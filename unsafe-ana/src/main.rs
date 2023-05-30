use clap::Parser;
use rand::Rng;
use std::{cell::RefCell, collections::HashMap, env, fs, io::Write, path::Path, process::Command};
use syn::{
    __private::quote::quote,
    visit::{self, Visit},
};
use walkdir::WalkDir;

thread_local! {
    // 0: union, 1: enum, 2: struct
    pub static TYPENUM: RefCell<HashMap<String, (u8, u32)>> = RefCell::new(HashMap::new());
}

#[derive(Parser, Debug)]
struct Args {
    /// rust project directory
    #[arg(value_parser = is_dir)]
    directory: String,

    /// count type define
    #[arg(long, default_value_t = false)]
    count_type_define: bool,
}

fn is_dir(s: &str) -> Result<String, String> {
    if !Path::new(s).exists() {
        return Err(format!("{} isn't a valid directory", s));
    }
    Ok(s.to_owned())
}

fn main() {
    let args = Args::parse();
    env::set_current_dir(&args.directory).expect("invalid dir");
    let mut func_num = 0;
    let mut unsafe_func_num = 0;
    let mut unsafe_line_num = 0;
    let mut total_line_num = 0;
    let mut safe_func_num = 0;
    let mut type_define_num = 0;
    let mut rng = rand::thread_rng();
    for entry in WalkDir::new("src").into_iter().filter_map(Result::ok) {
        let mut helper_func: syn::ItemFn = syn::parse_str("fn helper() {}").unwrap();
        let f_path = entry.path().to_string_lossy();
        if f_path.ends_with(".rs") {
            let input_code = fs::read_to_string(&*f_path).unwrap();
            total_line_num += input_code.bytes().filter(|c| *c == b'\n').count() + 1;
            let st = syn::parse_file(&input_code).unwrap();
            let mut visiter = Visitor::new(&mut helper_func);
            visiter.visit_file(&st);
            func_num += visiter.func_num;
            unsafe_func_num += visiter.unsafe_func_num;
            safe_func_num += visiter.safe_func_num;
            type_define_num += visiter.type_define_num;
        }
        let helper_code = syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![syn::Item::Fn(helper_func)],
        };
        let filename = format!("temp-{}.rs", rng.gen_range(0..10000000));
        let mut helper_file = fs::File::create(&filename).unwrap();
        helper_file
            .write_all(quote!(#helper_code).to_string().as_bytes())
            .unwrap();
        drop(helper_file);
        Command::new("rustfmt").arg(&filename).status().unwrap();
        let format_code = fs::read_to_string(&filename).unwrap();
        unsafe_line_num += format_code.bytes().filter(|c| *c == b'\n').count() - 2;
        fs::remove_file(&filename).unwrap();
    }
    println!("Total Function Number: {func_num}");
    println!("Unsafe Function Number: {unsafe_func_num}");
    println!("Safe Function Without Unsafe Block Number: {safe_func_num}");
    println!("Total Line Number: {total_line_num}");
    println!("Unsafe Line Number: {unsafe_line_num}");
    if args.count_type_define {
        println!("Type Define Number: {type_define_num}");
        println!("name,type,number");
        TYPENUM.with(|m| {
            m.borrow_mut().iter().for_each(|i| {
                println!(
                    "{},{},{}",
                    i.0,
                    match i.1 .0 {
                        0 => "union",
                        1 => "enum",
                        _ => "struct",
                    },
                    i.1 .1
                );
            })
        });
    }
}

struct Visitor<'a> {
    type_define_num: u32,
    func_num: u32,
    unsafe_func_num: u32,
    safe_func_num: u32,
    state: i32,
    helper_func: &'a mut syn::ItemFn,
}

impl<'a> Visitor<'a> {
    pub fn new(helper_func: &'a mut syn::ItemFn) -> Self {
        Self {
            type_define_num: 0,
            func_num: 0,
            unsafe_func_num: 0,
            safe_func_num: 0,
            state: 0,
            helper_func,
        }
    }
}

impl Visit<'_> for Visitor<'_> {
    fn visit_item(&mut self, i: &'_ syn::Item) {
        match i {
            syn::Item::Fn(func) => {
                self.state = 1;
                self.func_num += 1;
                if func.sig.unsafety.is_some() {
                    self.state = 2;
                    self.unsafe_func_num += 1;
                    for stmt in &func.block.stmts {
                        let mut nstmt = stmt.clone();
                        match &mut nstmt {
                            syn::Stmt::Local(_) => (),
                            syn::Stmt::Item(_) => (),
                            syn::Stmt::Expr(_, semi) => *semi = Some(syn::token::Semi::default()),
                            syn::Stmt::Macro(m) => m.semi_token = Some(syn::token::Semi::default()),
                        }
                        self.helper_func.block.stmts.push(nstmt);
                    }
                }
            }
            syn::Item::Union(item) => {
                self.type_define_num += 1;
                TYPENUM.with(|m| {
                    m.borrow_mut()
                        .entry(item.ident.to_string())
                        .or_insert((0, 0))
                        .1 += 1
                });
            }
            syn::Item::Enum(item) => {
                self.type_define_num += 1;
                TYPENUM.with(|m| {
                    m.borrow_mut()
                        .entry(item.ident.to_string())
                        .or_insert((1, 0))
                        .1 += 1;
                })
            }
            syn::Item::Struct(item) => {
                self.type_define_num += 1;
                TYPENUM.with(|m| {
                    m.borrow_mut()
                        .entry(item.ident.to_string())
                        .or_insert((2, 0))
                        .1 += 1;
                })
            }
            _ => {}
        }
        visit::visit_item(self, i);
        if self.state == 1 {
            self.safe_func_num += 1;
        }
        self.state = 0;
    }

    fn visit_expr(&mut self, i: &'_ syn::Expr) {
        match i {
            syn::Expr::Unsafe(expr) => {
                if self.state == 1 {
                    self.state = 2;
                }
                for stmt in &expr.block.stmts {
                    let mut nstmt = stmt.clone();
                    match &mut nstmt {
                        syn::Stmt::Local(_) => (),
                        syn::Stmt::Item(_) => (),
                        syn::Stmt::Expr(_, semi) => *semi = Some(syn::token::Semi::default()),
                        syn::Stmt::Macro(m) => m.semi_token = Some(syn::token::Semi::default()),
                    }
                    self.helper_func.block.stmts.push(nstmt);
                }
            }
            _ => (),
        }
        visit::visit_expr(self, i);
    }
}
