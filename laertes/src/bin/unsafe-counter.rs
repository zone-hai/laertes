#![feature(rustc_private)]
#![feature(non_ascii_idents)]

use crate::def_id::{DefId, LOCAL_CRATE};
use itertools::Itertools;
use laertes::{
    analysis,
    analysis::{
        commons::{Node, StructInfo},
        struct_info_pass::StructInfoPass,
    },
    colored::*,
    compiler_interface::*,
    config::*,
    constants::*,
    io::{FileIO, OutputMode},
    ptr_provenance::*,
    rustc_driver,
    rustc_hir::{intravisit::FnKind, *},
    rustc_lint::{LateContext, LateLintPass, LintPass},
    rustc_span::Span,
    solver::{SimpleTerm::LV, Term},
    types::Type,
    util::{self, powerset},
    util::{HashMap, HashSet},
    Name,
};
use lazy_static::lazy_static;
use std::{collections::BTreeSet, panic, sync::RwLock};

// Command-line and environment options for this command
#[derive(Debug)]
struct Flags {
    print_per_fn_stats: bool,
    count_indirect: bool,
    benchmark_name: String,
    non_limit: bool,
    // config file for introducing explicit poisons
    config_file: Option<String>,
}

impl Flags {
    fn new() -> Self {
        Flags {
            print_per_fn_stats: true,
            count_indirect: false,
            non_limit: false,
            benchmark_name: "(unknown)".to_string(),
            config_file: None,
        }
    }

    fn initialize(&mut self) {
        if let Ok(options) = std::env::var("LAERTES_OPTS") {
            let option_set: HashSet<&str> = options.split(",").collect();
            self.print_per_fn_stats = option_set.contains("print_per_fn_stats");
            self.count_indirect = option_set.contains("count_indirect");
            self.non_limit = option_set.contains("non_limit");
        }
        if let Ok(benchmark_name) = std::env::var("BENCHMARK") {
            self.benchmark_name = benchmark_name;
        }
        self.config_file = std::env::var("CONFIG").ok();
    }
}

lazy_static! {
    static ref FLAGS: RwLock<Flags> = RwLock::new(Flags::new());
    static ref PTR_VARS: RwLock<HashSet<Name>> = RwLock::new(HashSet::default());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
/// Different uses of pointers, used for collecting statistics
enum PtrUse {
    VoidPtr,
    Extern,
    PtrArith,
    Cast,
    Global,
    /// Values passed to function pointers
    FnPtr,
    /// Values explicitly marked as poisoned in the config
    ExplicitRaw,
    /// String literal-related casts
    StringLit,
}

/// Information about unsafety inside a function
#[derive(Clone)]
struct FnInfo {
    calls_prims_we_handle: bool,
    calls_other_unsafe_fns: bool,
    other_unsafe_fns_called: HashSet<Name>,
    /// which pointer uses the pointers in this function's body are
    /// involved in
    ptr_use: BTreeSet<PtrUse>,
    derefs_raw_ptr: bool,
    raw_ptr_arith: bool,
    is_unsafe: bool,
    other_unsafety: BTreeSet<UnsafeBehavior>,
    lines_of_code: Option<usize>,
}

impl FnInfo {
    pub fn new() -> Self {
        FnInfo {
            calls_prims_we_handle: false,
            calls_other_unsafe_fns: false,
            other_unsafe_fns_called: HashSet::default(),
            ptr_use: BTreeSet::new(),
            derefs_raw_ptr: false,
            raw_ptr_arith: false,
            is_unsafe: false,
            other_unsafety: BTreeSet::new(),
            lines_of_code: None,
        }
    }
}

impl Default for FnInfo {
    fn default() -> Self {
        FnInfo::new()
    }
}

/// Statistics we collect for different hypothetical cases
#[derive(Default)]
struct Statistics {
    unsafe_fns: usize,
}

/// A simple early lint pass
struct UnsafeCounter {
    unsafe_count: usize,
    fn_count: usize,
    total: Statistics,
    ptr_arith_only: Statistics,
    extern_call_only: Statistics,
    ptr_arith_and_extern_call: Statistics,
    perfect: Statistics,
    in_foreign_item: bool,
    fn_infos: HashMap<Name, FnInfo>,
    ptr_provenance: PtrProvenanceAnalysis,
    deref_counts: HashMap<BTreeSet<PtrUse>, usize>,
    ptr_decl_counts: HashMap<BTreeSet<PtrUse>, usize>,
    /// pointer or reference-typed variables
    ptr_vars: HashSet<Name>,
}

impl UnsafeCounter {
    fn new() -> Box<LatePass> {
        let ptr_provenance_results = analysis::result::<PtrProvenanceAnalysis>().unwrap();

        Box::new(UnsafeCounter {
            unsafe_count: 0,
            fn_count: 0,
            fn_infos: HashMap::default(),
            in_foreign_item: false,
            ptr_provenance: ptr_provenance_results,
            total: Statistics::default(),
            ptr_arith_only: Statistics::default(),
            extern_call_only: Statistics::default(),
            ptr_arith_and_extern_call: Statistics::default(),
            perfect: Statistics::default(),
            deref_counts: HashMap::default(),
            ptr_decl_counts: HashMap::default(),
            ptr_vars: HashSet::default(),
        })
    }

    fn ptr_use_for_expr(&self, expr_hir: HirId) -> BTreeSet<PtrUse> {
        if let Some(Term::S(LV(expr_loc))) = self.ptr_provenance.expr_to_term.get(&expr_hir) {
            self.ptr_use(expr_loc.clone())
        } else {
            BTreeSet::new()
        }
    }

    fn ptr_use(&self, term: Loc<Name>) -> BTreeSet<PtrUse> {
        let tainted_by_source_sink = |source, sink| {
            self.ptr_provenance.has_poison(&term, source)
                || self.ptr_provenance.has_poison(&term, sink)
        };

        let mut ptr_use = BTreeSet::new();
        if tainted_by_source_sink(PoisonKind::VoidSource, PoisonKind::VoidSink) {
            ptr_use.insert(PtrUse::VoidPtr);
        }
        if tainted_by_source_sink(PoisonKind::PtrArith, PoisonKind::PtrArithSink) {
            ptr_use.insert(PtrUse::PtrArith);
        }
        if tainted_by_source_sink(PoisonKind::PromotedSource, PoisonKind::PromotedSink) {
            ptr_use.insert(PtrUse::ExplicitRaw);
        }
        if tainted_by_source_sink(PoisonKind::ExternCallReturn, PoisonKind::ExternCallParam) {
            ptr_use.insert(PtrUse::Extern);
        }
        if tainted_by_source_sink(PoisonKind::CastSource, PoisonKind::CastSink) {
            ptr_use.insert(PtrUse::Cast);
        }
        if tainted_by_source_sink(PoisonKind::GlobalSource, PoisonKind::GlobalSink) {
            ptr_use.insert(PtrUse::Global);
        }
        if self
            .ptr_provenance
            .has_poison(&term, PoisonKind::StringLitSource)
        {
            ptr_use.insert(PtrUse::StringLit);
        }
        if FLAGS.read().unwrap().non_limit {
            if tainted_by_source_sink(PoisonKind::FnPtrSource, PoisonKind::FnPtrSink) {
                ptr_use.insert(PtrUse::FnPtr);
            }
        }

        ptr_use
    }

    fn fn_info(&mut self, fn_name: Name) -> &mut FnInfo {
        self.fn_infos.entry(fn_name).or_default()
    }

    /// Can calls to the argument function be handled
    fn can_be_handled(&self, fn_path: &[String]) -> bool {
        match fn_path.last() {
            None => return false, // The path is empty
            Some(s) => {
                if C_FNS_WE_HANDLE.contains(s) {
                    return true;
                }
            },
        };

        let path = fn_path
            .iter()
            .clone()
            .filter(|s| !s.is_empty())
            .map(|s| s.clone())
            .collect::<Vec<String>>()
            .join("::");
        RUST_FNS_WE_HANDLE.contains(&path)
    }

    fn get_def_path<'tcx>(&self, ctx: &LateContext<'tcx>, def_id: DefId) -> Vec<String> {
        ctx.get_def_path(def_id)
            .iter()
            .map(|segment| segment.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
    }

    fn get_def_qname<'tcx>(&self, ctx: &LateContext<'tcx>, def_id: DefId) -> String {
        self.get_def_path(ctx, def_id)
            .into_iter()
            .filter(|s| !s.is_empty())
            .join("::")
    }
}

impl LintPass for UnsafeCounter {
    fn name(&self) -> &'static str {
        "UnsafeCounter"
    }
}

fn unsafety_we_can_handle(unsafety: &BTreeSet<UnsafeBehavior>) -> bool {
    let mut unsafety = unsafety.clone();
    unsafety.remove(&UnsafeBehavior::RawPtrDeref);
    unsafety.remove(&UnsafeBehavior::Alloc);
    unsafety.is_empty()
}

impl<'tcx> LateLintPass<'tcx> for UnsafeCounter {
    fn check_fn_post(
        &mut self,
        ctx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        hir_id: HirId,
    ) {
        if span.in_derive_expansion() {
            return; // skip derived methods
        }

        let unsafety = match kind {
            FnKind::ItemFn(_ident, _generics, header, _visibility) => {
                self.fn_count += 1;
                header.unsafety
            },
            FnKind::Method(_ident, sig, _visibility) => {
                self.fn_count += 1;
                sig.header.unsafety
            },
            FnKind::Closure => {
                log::warn!("Skipping closure at {:?}", span);
                return;
            },
        };

        let def_id = DefId {
            krate: LOCAL_CRATE,
            index: ctx.tcx.hir().local_def_id(hir_id).local_def_index,
        };

        let def_qname = Name::from(self.get_def_qname(ctx, def_id));

        if let Unsafety::Unsafe = unsafety {
            self.unsafe_count += 1;
            self.fn_info(def_qname.clone()).is_unsafe = true;
        }

        let typeck = ctx.typeck_results();

        if !def_qname.contains("laertes_xt") {
            for (i, param) in body.params.iter().enumerate() {
                if typeck.pat_ty(param.pat).is_unsafe_ptr() {
                    // this is a raw pointer, update counts
                    let ptr_use = self.ptr_use(Loc::Param(def_qname.clone(), i));

                    self.fn_infos
                        .get_mut(&def_qname)
                        .unwrap()
                        .ptr_use
                        .extend(ptr_use.into_iter());
                }
            }
        }
    }

    fn check_foreign_item(&mut self, _: &LateContext<'tcx>, _: &'tcx ForeignItem<'tcx>) {
        self.in_foreign_item = true;
    }

    fn check_foreign_item_post(&mut self, _: &LateContext<'tcx>, _: &'tcx ForeignItem<'tcx>) {
        self.in_foreign_item = false;
    }

    fn check_pat(&mut self, ctx: &LateContext<'tcx>, pat: &Pat<'tcx>) {
        if self.in_foreign_item {
            panic!("in foreign item when processing pattern {:?}", pat);
        }
        let owner_fn = self.get_def_qname(ctx, ctx.typeck_results().hir_owner.to_def_id());

        if owner_fn.contains("laertes_xt::") || is_laertes_helper(owner_fn.as_ref()) {
            return;
        }
        let in_bitfield_helper = owner_fn.contains("::bitfields::")
            || owner_fn.ends_with("xmlschemastypes::get_bits")
            || owner_fn.ends_with("xmlschemastypes::set_bits");
        if in_bitfield_helper || owner_fn.ends_with("::main") {
            return;
        }
        pat.walk_always(|p| {
            if let PatKind::Binding(_, _, name, _) = &p.kind {
                let ty = ctx.typeck_results().pat_ty(p);
                let qname = Name::from(format!("{}::{}", owner_fn, name.as_str()));
                if ty.is_unsafe_ptr() {
                    let loc = Loc::Var(qname.clone());
                    let ptr_use = self.ptr_use(loc.clone());

                    /*if ptr_use.contains(&PtrUse::PtrArith)
                        && !ptr_use.contains(&PtrUse::Extern)
                        && !ptr_use.contains(&PtrUse::VoidPtr)
                    {
                        println!(
                            "ARITH PTR: {} ; poison: {:?} at {:?}",
                            qname,
                            self.ptr_provenance.poisons(&loc),
                            p.span
                        );
                    }*/

                    // if true {
                    //     println!(
                    //         "Non-limit ptr: {} ; poison: {:?} at {:?}",
                    //         qname,
                    //         self.ptr_provenance.poisons(&loc),
                    //         p.span
                    //     );
                    // }

                    *self.ptr_decl_counts.entry(ptr_use.clone()).or_default() += 1;
                    self.fn_info(Name::from(owner_fn.clone()))
                        .ptr_use
                        .extend(ptr_use.into_iter());
                }
                let typ = Type::from_ty(ctx, ty);
                if typ.is_ptr_like() {
                    self.ptr_vars.insert(qname.clone());
                }
            }
        });
    }

    fn check_expr_post(&mut self, ctx: &LateContext<'tcx>, expr: &Expr) {
        let typeck = ctx.typeck_results();
        let hir_map = ctx.tcx.hir();
        let body_id = hir_map.enclosing_body_owner(expr.hir_id);

        let in_fn = matches!(hir_map.body_owner_kind(body_id), BodyOwnerKind::Fn);

        if !in_fn {
            return; // Do not collect statistics about values outside
            // functions
        }

        let caller_def_id = DefId {
            krate: LOCAL_CRATE,
            index: body_id.owner.local_def_index,
        };

        let caller = Name::from(self.get_def_qname(ctx, caller_def_id));

        match expr.kind {
            ExprKind::Unary(UnOp::Deref, ptr) => {
                if typeck.expr_ty(ptr).is_unsafe_ptr() {
                    let ptr_use = self.ptr_use_for_expr(ptr.hir_id);

                    *self.deref_counts.entry(ptr_use.clone()).or_default() += 1;
                    self.fn_info(caller).ptr_use.extend(ptr_use.into_iter());
                }
            },
            ExprKind::Call(fun, _args) => {
                let callee_sig = typeck.expr_ty(fun).fn_sig(ctx.tcx);

                if let ExprKind::Path(QPath::Resolved(_, path)) = fun.kind {
                    if let Some(def_id) = path.res.opt_def_id() {
                        let callee_path = self.get_def_path(ctx, def_id);
                        let callee_name = Name::from(self.get_def_qname(ctx, def_id));

                        if callee_sig.unsafety() == Unsafety::Unsafe {
                            if self.can_be_handled(&callee_path) {
                                self.fn_info(caller.clone()).calls_prims_we_handle = true;
                            } else {
                                self.fn_info(caller.clone())
                                    .other_unsafe_fns_called
                                    .insert(callee_name);
                            }
                        }
                    }
                }
            },
            ExprKind::MethodCall(..) => {
                // There are no method calls with unsafe behavior in
                // the programs we handle, so we don't need to merge
                // any information.
            },
            _ => (),
        }
    }

    fn check_crate_post(&mut self, _: &LateContext<'tcx>) {
        // drop calls to other unsafe functions from the same crate
        let declared_unsafe_fns = self
            .fn_infos
            .iter()
            .filter_map(|(fn_name, fn_info)| {
                if fn_info.is_unsafe {
                    Some(fn_name.to_owned())
                } else {
                    None
                }
            })
            .collect::<HashSet<Name>>();
        for (_fn_name, fn_info) in self.fn_infos.iter_mut() {
            fn_info
                .other_unsafe_fns_called
                .retain(|f| !declared_unsafe_fns.contains(f));
            fn_info.calls_other_unsafe_fns = !fn_info.other_unsafe_fns_called.is_empty();
            /*if fn_info.calls_other_unsafe_fns {
                println!("{} is unsafe because it calls:", fn_name.bold());
                for f in fn_info.other_unsafe_fns_called.iter() {
                    println!("\t- {}", f);
                }
            }*/
        }

        // count only unsafe functions
        self.fn_infos.retain(|_, fn_info| fn_info.is_unsafe);

        let call_graph = analysis::result::<CallGraph>().unwrap();

        // record other unsafe behavior transitively
        for (callee, callers) in call_graph.transitive_callers().unwrap() {
            let unsafe_behavior = call_graph
                .unsafe_behavior()
                .get(callee)
                .cloned()
                .unwrap_or(BTreeSet::new());
            if unsafe_behavior.is_empty() {
                continue;
            }

            for caller in callers {
                self.fn_infos
                    .get_mut(caller)
                    .map(|fn_info| fn_info.other_unsafety.extend(unsafe_behavior.clone()));
            }
        }

        // record unsafe behavior that occurs in function body (not through callees)
        for (fn_name, unsafe_behavior) in call_graph.unsafe_behavior() {
            let unsafe_behavior = unsafe_behavior.clone();
            self.fn_infos
                .get_mut(fn_name)
                .map(|fn_info| fn_info.other_unsafety.extend(unsafe_behavior));
        }

        let mut tainted_fns = HashSet::default();

        for loc in &self.ptr_provenance.poisons[&PoisonKind::PtrArith] {
            if loc.is_from_program() {
                if self.ptr_provenance.owners.contains_key(&loc) {
                    for f in self.ptr_provenance.owners[&loc].iter() {
                        tainted_fns.insert(f.clone());
                    }
                }
            }
        }

        
        println!("function breakdown:");
        println!(
            "total function count (foreign functions excluded): {}",
            format!("{}", self.fn_count).bold().red()
        );
        println!(
            "unsafe function count (todo: count inner unsafe blocks): {}",
            format!("{}", self.unsafe_count).bold().red()
        );
        self.total.unsafe_fns = self.unsafe_count;
        self.perfect.unsafe_fns = self
            .fn_infos
            .iter()
            .filter(|(_, fn_info)| unsafety_we_can_handle(&fn_info.other_unsafety))
            .count();

        let fns_with_other_unsafety = self
            .fn_infos
            .values()
            .filter(|fn_info| !unsafety_we_can_handle(&fn_info.other_unsafety))
            .count();

        let fns_with_raw_ptrs = self
            .fn_infos
            .iter()
            .filter(|(_, fn_info)| {
                !fn_info.raw_ptr_arith
                    && unsafety_we_can_handle(&fn_info.other_unsafety)
                    && fn_info
                        .other_unsafety
                        .contains(&UnsafeBehavior::RawPtrDeref)
            })
            .count();

        let fns_that_call_prims_we_handle = self
            .fn_infos
            .iter()
            .filter(|(_, fn_info)| {
                !fn_info.raw_ptr_arith
                    && unsafety_we_can_handle(&fn_info.other_unsafety)
                    && fn_info.calls_prims_we_handle
            })
            .count();

        self.ptr_arith_only.unsafe_fns = self
            .fn_infos
            .iter()
            .filter(|(_, fn_info)| fn_info.raw_ptr_arith && !fn_info.calls_other_unsafe_fns)
            .count();

        self.ptr_arith_and_extern_call.unsafe_fns = self
            .fn_infos
            .iter()
            .filter(|(name, fn_info)| {
                fn_info.raw_ptr_arith || {
                    if let Some(callees) = call_graph.closure().unwrap().get(name) {
                        callees.iter().any(|callee| tainted_fns.contains(callee))
                    } else {
                        false
                    }
                }
            })
            .count();

        self.ptr_arith_and_extern_call.unsafe_fns = self
            .fn_infos
            .iter()
            .filter(|(_, fn_info)| fn_info.calls_other_unsafe_fns)
            .count();

        self.extern_call_only.unsafe_fns = self
            .fn_infos
            .iter()
            .filter(|(_, fn_info)| fn_info.calls_other_unsafe_fns && !fn_info.raw_ptr_arith)
            .count();

        println!(
            "functions we can handle: {}\n\t- calls prims we handle: {}\n\t- derefs raw ptrs: {}",
            format!("{}", self.perfect.unsafe_fns).bold().red(),
            fns_that_call_prims_we_handle,
            fns_with_raw_ptrs
        );

        println!(
            "functions with other unsafety: {}",
            format!("{}", fns_with_other_unsafety).bold().magenta()
        );
        

        let benchmark_name = FLAGS.read().unwrap().benchmark_name.clone();

        println!("{}", "raw pointer CSV".bold().yellow());

        // let ptr_related_ub = vec![
        //     UnsafeBehavior::ExternCall,
        //     UnsafeBehavior::Alloc,
        //     UnsafeBehavior::RawPtrDeref,
        // ]
        // .into_iter()
        // .collect::<BTreeSet<UnsafeBehavior>>();
        let ptr_use_sets = vec![
            vec![PtrUse::VoidPtr],
            vec![PtrUse::ExplicitRaw],
            vec![PtrUse::PtrArith],
            vec![PtrUse::Global],
            vec![PtrUse::Cast],
            vec![PtrUse::VoidPtr],
            vec![PtrUse::StringLit],
            vec![PtrUse::Extern],
            vec![],
        ]
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect::<Vec<BTreeSet<PtrUse>>>();
        print!("Benchmark,Statistic");
        for set in &ptr_use_sets {
            print!(",{:?}", set);
        }
        print!("\n{},NonUniqueDecls", benchmark_name);
        for set in &ptr_use_sets {
            print!(
                ",{}",
                self.ptr_decl_counts
                    .iter()
                    .filter(|(s, _)| set.is_subset(s) && (set.is_empty() || &set != s))
                    .map(|(_, count)| count)
                    .sum::<usize>()
            );
        }
        print!("\n{},DeclsExact", benchmark_name);
        for set in &ptr_use_sets {
            print!(",{}", self.ptr_decl_counts.get(set).cloned().unwrap_or(0));
        }

        print!("\n{},NonUniqueDerefs", benchmark_name);
        for set in &ptr_use_sets {
            print!(
                ",{}",
                self.deref_counts
                    .iter()
                    .filter(|(s, _)| set.is_subset(s) && (set.is_empty() || &set != s))
                    .map(|(_, count)| count)
                    .sum::<usize>()
            );
        }
        print!("\n{},DerefsExact", benchmark_name);
        for set in &ptr_use_sets {
            print!(",{}", self.deref_counts.get(set).cloned().unwrap_or(0));
        }

        // print!("\n{},NonUniqueFns", benchmark_name);
        // for set in &ptr_use_sets {
        //    print!(
        //        ",{}",
        //        self.fn_infos
        //            .iter()
        //            .filter(|(f, fn_info)| {
        //                // f has the interesting behavior
        //                set.is_subset(&fn_info.ptr_use) &&
        //                (set.is_empty() || set != &fn_info.ptr_use) &&
        //                // f does not have pointer-unrelated unsafe behavior
        //                    call_graph.unsafe_behavior().get(f).map_or(false, |s| s.is_subset(&ptr_related_ub))
        //            })
        //            .count()
        //    );
        // }

        // print!("\n{},FnsExact", benchmark_name);
        // for set in &ptr_use_sets {
        //    print!(
        //        ",{}",
        //        self.fn_infos
        //            .iter()
        //            .filter(|(f, fn_info)| {
        //                    // f has the interesting behavior
        //                    set == &fn_info.ptr_use &&
        //                // f does not have pointer-unrelated unsafe behavior
        //                    call_graph.unsafe_behavior().get(f).map_or(false, |s| s.is_subset(&ptr_related_ub))
        //            })
        //            .count()
        //    );
        // }
        println!();

        // println!("{}", "unsafe behavior CSV".bold().blue());
        // print!("Benchmark,Statistic");
        // for ub in powerset(&ALL_UNSAFE_BEHAVIOR) {
        //     print!(",\"{:?}\"", ub)
        // }
        // print!("\n{},Occurrences", benchmark_name);
        // for ubs in powerset(&ALL_UNSAFE_BEHAVIOR) {
        //     print!(
        //         ",{}",
        //         ubs.iter()
        //             .map(|ub| call_graph.ub_count().get(ub).cloned().unwrap_or_default())
        //             .sum::<usize>()
        //     );
        // }
        print!("\n{},ExactUnsafeFns", benchmark_name);
        for ub in powerset(&ALL_UNSAFE_BEHAVIOR) {
            let ub_set = ub.into_iter().collect::<BTreeSet<UnsafeBehavior>>();
            print!(
                ",{}",
                call_graph
                    .unsafe_behavior()
                    .iter()
                    .filter(
                        |(name, ubs)| self.fn_infos.get(*name).map_or(false, |f| f.is_unsafe)
                            && **ubs == ub_set
                    )
                    .count()
            );
        }
        print!("\n{},UnsafeFns", benchmark_name);
        for ub in powerset(&ALL_UNSAFE_BEHAVIOR) {
            let ub_set = ub.into_iter().collect::<BTreeSet<UnsafeBehavior>>();
            print!(
                ",{}",
                call_graph
                    .unsafe_behavior()
                    .iter()
                    .filter(
                        |(name, ubs)| self.fn_infos.get(*name).map_or(false, |f| f.is_unsafe)
                            && ubs.is_subset(&ub_set)
                    )
                    .count()
            );
        }
        println!("");

        // save pointer variable names
        PTR_VARS.write().unwrap().extend(self.ptr_vars.drain());
    }
}

pub fn main() {
    // This is our logger
    env_logger::init();
    // Initialize rustc's logger as well
    rustc_driver::init_rustc_env_logger();
    FLAGS.write().unwrap().initialize();
    println!("environment variable flags: {:?}", FLAGS.read().unwrap());

    let non_limit = FLAGS.read().unwrap().non_limit;
    LIMIT_STUDY.store(!non_limit, std::sync::atomic::Ordering::SeqCst);
    EMULATE_LIFETIME_ONLY.store(non_limit, std::sync::atomic::Ordering::SeqCst);

    if let Some(config_file) = FLAGS.read().unwrap().config_file.as_ref() {
        let cfg = laertes::Config {
            output_path: None,
            output_mode: OutputMode::Overwrite,
        };

        let file_io = FileIO::new(&cfg);
        println!("Loading the initial configuration from {}", config_file);
        *CONFIG.write().unwrap() = laertes::ron::from_str(
            file_io
                .read_file(std::path::Path::new(config_file))
                .unwrap()
                .as_str(),
        )
        .unwrap();
    }

    // TODO(maemre): We are running rustc for each lint pass
    // separately, because of data dependencies. Run rustc fewer times
    // and run the passes independent of it.

    // Run the pointer provenance pass
    run_compiler(&vec![], vec![PtrProvenancePass::new]);
    // Run the struct info pass
    run_compiler(&vec![], vec![StructInfoPass::new]);

    // Propagate externs along struct info
    let struct_info = analysis::result::<StructInfo>().unwrap();
    let mut ptr_provenance = analysis::result::<PtrProvenanceAnalysis>().unwrap();
    // add taint for fields of structs that occur in external APIs
    for node in &struct_info.occurs_in_external_apis {
        let analysis = &mut ptr_provenance;
        match node {
            Node::Struct(struct_name) => {
                let add_extern_poison = |(f_name, f_ty): &(Name, Type)| {
                    let field_loc = Loc::Access(struct_name.clone(), f_name.clone());
                    // if this is a complex type containing a function pointer, then
                    // we need to create the relevant λ locations in case this
                    // function pointer is never instantiated with a concrete
                    // function in the library we are rewriting.
                    analysis.create_dummy_locs(field_loc.clone(), f_ty, true);
                    analysis.add_poison(PoisonKind::ExternCallParam, field_loc.clone());
                    analysis.add_poison(PoisonKind::ExternCallReturn, field_loc);
                };

                if let Some(struct_def) = struct_info.struct_defs.get(struct_name) {
                    // The case for when this is a struct
                    struct_def.fields.iter().for_each(add_extern_poison);
                } else if struct_info.union_defs.contains_key(struct_name) {
                    // The case for when this is a union, we don't translate
                    // unions so ignore them
                    log::warn!(
                        "union {} is used in external APIs, not doing anything since we don't translate unions",
                        struct_name
                    );
                }
            },
            Node::FnPtr(loc, sig) => {
                let analysis = &mut ptr_provenance;
                // Go through all types pointed to by this function pointer and poison them
                // TODO: handle varargs
                let param_locs = sig
                    .param_types
                    .iter()
                    .map(|ty| {
                        let param_loc = Loc::fresh();
                        analysis.create_dummy_locs(param_loc.clone(), ty, false);
                        analysis.add_poison(PoisonKind::ExternCallParam, param_loc.clone());
                        param_loc.to_st()
                    })
                    .collect::<Vec<laertes::solver::SimpleTerm<Loc<Name>>>>();
                let ret_loc = {
                    let loc = Loc::fresh();
                    analysis.create_dummy_locs(loc.clone(), sig.ret_type.as_ref(), true);
                    analysis.add_poison(PoisonKind::ExternCallReturn, loc.clone());
                    loc.to_st()
                };
                let lambda = mk_lambda(param_locs, ret_loc);
                analysis.add_flow(loc.clone().to_term(), lambda);
            },
            _ => unreachable!(),
        }
    }
    // solve the taint analysis again, then update the global record
    ptr_provenance.propagate_occurrence_poisons_through_lambdas();
    ptr_provenance.solve().unwrap();
    analysis::replace(Box::new(ptr_provenance.clone()));

    // Run the unsafe counter pass
    run_compiler(&vec![], vec![UnsafeCounter::new]);

    // Query the equivalence classes
    let ptr_vars = PTR_VARS.read().unwrap();
    let mut sizes = Vec::<usize>::new();
    for (_, eq_class) in ptr_provenance.constraints.compute_eq_classes() {
        let size = eq_class
            .iter()
            .filter(|loc| {
                if let Loc::Var(name) = loc {
                    ptr_vars.contains(name)
                } else {
                    false
                }
            })
            .count();
        sizes.push(size);
    }
    sizes.sort();
    // println!("non-empty pointer eq class sizes:");
    // for size in &sizes {
    //     if *size > 0 {
    //         println!("{}", size);
    //     }
    // }
    println!(
        "there are {} non-empty eq classes",
        sizes.iter().filter(|x| **x > 0).count()
    );
    println!("total number of pointers: {}", sizes.into_iter().filter(|x| *x > 0).sum::<usize>());

    util::report_profiling_results();
}
