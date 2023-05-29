use ::libc;
extern "C" {
    fn memset(
        _: * mut core::ffi::c_void,
        _: i32,
        _: u64,
    ) -> * mut core::ffi::c_void;
    
    
    
    
}
pub use crate::src::globals::__xmlGenericError;
pub use crate::src::globals::__xmlGenericErrorContext;
pub use crate::src::globals::xmlFree;
pub use crate::src::globals::xmlMalloc;
pub type size_t = u64;
pub type xmlFreeFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C"  fn(_: u64,) -> * mut core::ffi::c_void>;
pub type xmlGenericErrorFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLink {
    pub next: * mut crate::src::list::_xmlLink,
    pub prev: * mut crate::src::list::_xmlLink,
    pub data: * mut core::ffi::c_void,
}
impl _xmlLink {
    pub const fn new() -> Self {
        _xmlLink {
        next: (0 as * mut crate::src::list::_xmlLink),
        prev: (0 as * mut crate::src::list::_xmlLink),
        data: (0 as * mut core::ffi::c_void)
        }
    }
}

impl std::default::Default for _xmlLink {
    fn default() -> Self { _xmlLink::new() }
}

pub type xmlLink = crate::src::list::_xmlLink;
pub type xmlLinkPtr = * mut crate::src::list::_xmlLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlList {
    pub sentinel: * mut crate::src::list::_xmlLink,
    pub linkDeallocator: Option<unsafe extern "C"  fn(_: * mut crate::src::list::_xmlLink,) -> ()>,
    pub linkCompare: Option<unsafe extern "C"  fn(_: * const core::ffi::c_void,_: * const core::ffi::c_void,) -> i32>,
}
impl _xmlList {
    pub const fn new() -> Self {
        _xmlList {
        sentinel: (0 as * mut crate::src::list::_xmlLink),
        linkDeallocator: None,
        linkCompare: None
        }
    }
}

impl std::default::Default for _xmlList {
    fn default() -> Self { _xmlList::new() }
}

pub type xmlList = crate::src::list::_xmlList;
pub type xmlListPtr = * mut crate::src::list::_xmlList;
pub type xmlListDeallocator = Option<unsafe extern "C"  fn(_: * mut crate::src::list::_xmlLink,) -> ()>;
pub type xmlListDataCompare = Option<unsafe extern "C"  fn(_: * const core::ffi::c_void,_: * const core::ffi::c_void,) -> i32>;
pub type xmlListWalker = Option<unsafe extern "C"  fn(_: * const core::ffi::c_void,_: * mut core::ffi::c_void,) -> i32>;
unsafe extern "C" fn xmlLinkDeallocator(mut l: * mut crate::src::list::_xmlList, mut lk: * mut crate::src::list::_xmlLink) {
    let ref mut fresh0 = (*(*lk).prev).next;
    *fresh0 = (*lk).next;
    let ref mut fresh1 = (*(*lk).next).prev;
    *fresh1 = (*lk).prev;
    if ((*l).linkDeallocator).is_some() {
        ((*l).linkDeallocator).expect("non-null function pointer")(lk);
    }
    xmlFree.expect("non-null function pointer")(lk as *mut libc::c_void);
}
 extern "C" fn xmlLinkCompare(
    mut data0: * const core::ffi::c_void,
    mut data1: * const core::ffi::c_void,
) -> i32 {
    if data0 < data1 {
        return -(1 as i32)
    } else {
        if data0 == data1 {
            return 0 as i32;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlListLowerSearch(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> * mut crate::src::list::_xmlLink {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel
        && ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            < 0 as i32
    {
        lk = (*lk).next;
    }
    return lk;
}
unsafe extern "C" fn xmlListHigherSearch(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> * mut crate::src::list::_xmlLink {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = (*(*l).sentinel).prev;
    while lk != (*l).sentinel
        && ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            > 0 as i32
    {
        lk = (*lk).prev;
    }
    return lk;
}
unsafe extern "C" fn xmlListLinkSearch(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> * mut crate::src::list::_xmlLink {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = xmlListLowerSearch(l, data);
    if lk == (*l).sentinel {
        return 0 as xmlLinkPtr
    } else {
        if ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            == 0 as i32
        {
            return lk;
        }
        return 0 as xmlLinkPtr;
    };
}
unsafe extern "C" fn xmlListLinkReverseSearch(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> * mut crate::src::list::_xmlLink {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = xmlListHigherSearch(l, data);
    if lk == (*l).sentinel {
        return 0 as xmlLinkPtr
    } else {
        if ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            == 0 as i32
        {
            return lk;
        }
        return 0 as xmlLinkPtr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlListCreate(
    mut deallocator: Option<unsafe extern "C"  fn(_: * mut crate::src::list::_xmlLink,) -> ()>,
    mut compare: Option<unsafe extern "C"  fn(_: * const core::ffi::c_void,_: * const core::ffi::c_void,) -> i32>,
) -> * mut crate::src::list::_xmlList {
    let mut l: * mut crate::src::list::_xmlList = 0 as *mut xmlList;
    l = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlList>() as u64) as xmlListPtr;
    if l.is_null() {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Cannot initialize memory for list\0" as *const u8 as *const i8,
        );
        return 0 as xmlListPtr;
    }
    memset(
        l as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlList>() as u64,
    );
    let ref mut fresh2 = (*l).sentinel;
    *fresh2 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if (*fresh2).is_null() {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Cannot initialize memory for sentinel\0" as *const u8
                as *const i8,
        );
        xmlFree.expect("non-null function pointer")(l as *mut libc::c_void);
        return 0 as xmlListPtr;
    }
    let ref mut fresh3 = (*(*l).sentinel).next;
    *fresh3 = (*l).sentinel;
    let ref mut fresh4 = (*(*l).sentinel).prev;
    *fresh4 = (*l).sentinel;
    let ref mut fresh5 = (*(*l).sentinel).data;
    *fresh5 = 0 as *mut libc::c_void;
    if deallocator.is_some() {
        let ref mut fresh6 = (*l).linkDeallocator;
        *fresh6 = deallocator;
    }
    if compare.is_some() {
        let ref mut fresh7 = (*l).linkCompare;
        *fresh7 = compare;
    } else {
        let ref mut fresh8 = (*l).linkCompare;
        *fresh8 = Some(
            xmlLinkCompare,
        );
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSearch(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> * mut core::ffi::c_void {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as *mut libc::c_void;
    }
    lk = xmlListLinkSearch(l, data);
    if !lk.is_null() {
        return (*lk).data;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverseSearch(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> * mut core::ffi::c_void {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as *mut libc::c_void;
    }
    lk = xmlListLinkReverseSearch(l, data);
    if !lk.is_null() {
        return (*lk).data;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListInsert(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> i32 {
    let mut lkPlace: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    let mut lkNew: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 1 as i32;
    }
    lkPlace = xmlListLowerSearch(l, data);
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const i8,
        );
        return 1 as i32;
    }
    let ref mut fresh9 = (*lkNew).data;
    *fresh9 = data;
    lkPlace = (*lkPlace).prev;
    let ref mut fresh10 = (*lkNew).next;
    *fresh10 = (*lkPlace).next;
    let ref mut fresh11 = (*(*lkPlace).next).prev;
    *fresh11 = lkNew;
    let ref mut fresh12 = (*lkPlace).next;
    *fresh12 = lkNew;
    let ref mut fresh13 = (*lkNew).prev;
    *fresh13 = lkPlace;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListAppend(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> i32 {
    let mut lkPlace: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    let mut lkNew: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 1 as i32;
    }
    lkPlace = xmlListHigherSearch(l, data);
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const i8,
        );
        return 1 as i32;
    }
    let ref mut fresh14 = (*lkNew).data;
    *fresh14 = data;
    let ref mut fresh15 = (*lkNew).next;
    *fresh15 = (*lkPlace).next;
    let ref mut fresh16 = (*(*lkPlace).next).prev;
    *fresh16 = lkNew;
    let ref mut fresh17 = (*lkPlace).next;
    *fresh17 = lkNew;
    let ref mut fresh18 = (*lkNew).prev;
    *fresh18 = lkPlace;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListDelete(mut l: * mut crate::src::list::_xmlList) {
    if l.is_null() {
        return;
    }
    xmlListClear(l);
    xmlFree.expect("non-null function pointer")((*l).sentinel as *mut libc::c_void);
    xmlFree.expect("non-null function pointer")(l as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveFirst(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> i32 {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as i32;
    }
    lk = xmlListLinkSearch(l, data);
    if !lk.is_null() {
        xmlLinkDeallocator(l, lk);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveLast(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> i32 {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as i32;
    }
    lk = xmlListLinkReverseSearch(l, data);
    if !lk.is_null() {
        xmlLinkDeallocator(l, lk);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveAll(
    mut l: * mut crate::src::list::_xmlList,
    mut data: * mut core::ffi::c_void,
) -> i32 {
    let mut count: i32 = 0 as i32;
    if l.is_null() {
        return 0 as i32;
    }
    while xmlListRemoveFirst(l, data) != 0 {
        count += 1;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListClear(mut l: * mut crate::src::list::_xmlList) {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() {
        return;
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        let mut next: * mut crate::src::list::_xmlLink = (*lk).next;
        xmlLinkDeallocator(l, lk);
        lk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEmpty(mut l: * mut crate::src::list::_xmlList) -> i32 {
    if l.is_null() {
        return -(1 as i32);
    }
    return ((*(*l).sentinel).next == (*l).sentinel) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListFront<'a1>(mut l: Option<&'a1 mut crate::src::list::_xmlList>) -> * mut crate::src::list::_xmlLink {
    if borrow(& l).is_none() {
        return 0 as xmlLinkPtr;
    }
    return (*(*(borrow_mut(&mut l)).unwrap()).sentinel).next;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEnd<'a1>(mut l: Option<&'a1 mut crate::src::list::_xmlList>) -> * mut crate::src::list::_xmlLink {
    if borrow(& l).is_none() {
        return 0 as xmlLinkPtr;
    }
    return (*(*(borrow_mut(&mut l)).unwrap()).sentinel).prev;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSize<'a1>(mut l: Option<&'a1 mut crate::src::list::_xmlList>) -> i32 {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    let mut count: i32 = 0 as i32;
    if borrow(& l).is_none() {
        return -(1 as i32);
    }
    lk = (*(*(borrow_mut(&mut l)).unwrap()).sentinel).next;
    while lk != (*(borrow(& l)).unwrap()).sentinel {
        lk = (*lk).next;
        count += 1;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopFront(mut l: * mut crate::src::list::_xmlList) {
    if xmlListEmpty(l) == 0 {
        xmlLinkDeallocator(l, (*(*l).sentinel).next);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopBack(mut l: * mut crate::src::list::_xmlList) {
    if xmlListEmpty(l) == 0 {
        xmlLinkDeallocator(l, (*(*l).sentinel).prev);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPushFront<'a1>(
    mut l: Option<&'a1 mut crate::src::list::_xmlList>,
    mut data: * mut core::ffi::c_void,
) -> i32 {
    let mut lkPlace: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    let mut lkNew: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if borrow(& l).is_none() {
        return 0 as i32;
    }
    lkPlace = (*(borrow_mut(&mut l)).unwrap()).sentinel;
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const i8,
        );
        return 0 as i32;
    }
    let ref mut fresh19 = (*lkNew).data;
    *fresh19 = data;
    let ref mut fresh20 = (*lkNew).next;
    *fresh20 = (*lkPlace).next;
    let ref mut fresh21 = (*(*lkPlace).next).prev;
    *fresh21 = lkNew;
    let ref mut fresh22 = (*lkPlace).next;
    *fresh22 = lkNew;
    let ref mut fresh23 = (*lkNew).prev;
    *fresh23 = lkPlace;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPushBack<'a1>(
    mut l: Option<&'a1 mut crate::src::list::_xmlList>,
    mut data: * mut core::ffi::c_void,
) -> i32 {
    let mut lkPlace: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    let mut lkNew: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if borrow(& l).is_none() {
        return 0 as i32;
    }
    lkPlace = (*(*(borrow_mut(&mut l)).unwrap()).sentinel).prev;
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const i8,
        );
        return 0 as i32;
    }
    let ref mut fresh24 = (*lkNew).data;
    *fresh24 = data;
    let ref mut fresh25 = (*lkNew).next;
    *fresh25 = (*lkPlace).next;
    let ref mut fresh26 = (*(*lkPlace).next).prev;
    *fresh26 = lkNew;
    let ref mut fresh27 = (*lkPlace).next;
    *fresh27 = lkNew;
    let ref mut fresh28 = (*lkNew).prev;
    *fresh28 = lkPlace;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlLinkGetData<'a1>(mut lk: Option<&'a1 mut crate::src::list::_xmlLink>) -> * mut core::ffi::c_void {
    if borrow(& lk).is_none() {
        return 0 as *mut libc::c_void;
    }
    return (*(borrow_mut(&mut lk)).unwrap()).data;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverse<'a1>(mut l: Option<&'a1 mut crate::src::list::_xmlList>) {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    let mut lkPrev: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if borrow(& l).is_none() {
        return;
    }
    lkPrev = (*(borrow_mut(&mut l)).unwrap()).sentinel;
    lk = (*(*(borrow_mut(&mut l)).unwrap()).sentinel).next;
    while lk != (*(borrow(& l)).unwrap()).sentinel {
        let ref mut fresh29 = (*lkPrev).next;
        *fresh29 = (*lkPrev).prev;
        let ref mut fresh30 = (*lkPrev).prev;
        *fresh30 = lk;
        lkPrev = lk;
        lk = (*lk).next;
    }
    let ref mut fresh31 = (*lkPrev).next;
    *fresh31 = (*lkPrev).prev;
    let ref mut fresh32 = (*lkPrev).prev;
    *fresh32 = lk;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSort(mut l: * mut crate::src::list::_xmlList) {
    let mut lTemp: * mut crate::src::list::_xmlList = 0 as *mut xmlList;
    if l.is_null() {
        return;
    }
    if xmlListEmpty(l) != 0 {
        return;
    }
    lTemp = xmlListDup(l);
    if lTemp.is_null() {
        return;
    }
    xmlListClear(l);
    xmlListMerge(l, lTemp);
    xmlListDelete(lTemp);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListWalk(
    mut l: * mut crate::src::list::_xmlList,
    mut walker: Option<unsafe extern "C"  fn(_: * const core::ffi::c_void,_: * mut core::ffi::c_void,) -> i32>,
    mut user: * mut core::ffi::c_void,
) {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if l.is_null() || walker.is_none() {
        return;
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        if walker.expect("non-null function pointer")((*lk).data, user)
            == 0 as i32
        {
            break;
        }
        lk = (*lk).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverseWalk<'a1>(
    mut l: Option<&'a1 mut crate::src::list::_xmlList>,
    mut walker: Option<unsafe extern "C"  fn(_: * const core::ffi::c_void,_: * mut core::ffi::c_void,) -> i32>,
    mut user: * mut core::ffi::c_void,
) {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if borrow(& l).is_none() || walker.is_none() {
        return;
    }
    lk = (*(*(borrow_mut(&mut l)).unwrap()).sentinel).prev;
    while lk != (*(borrow(& l)).unwrap()).sentinel {
        if walker.expect("non-null function pointer")((*lk).data, user)
            == 0 as i32
        {
            break;
        }
        lk = (*lk).prev;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListMerge(mut l1: * mut crate::src::list::_xmlList, mut l2: * mut crate::src::list::_xmlList) {
    xmlListCopy(l1, l2);
    xmlListClear(l2);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListDup(old: * mut crate::src::list::_xmlList) -> * mut crate::src::list::_xmlList {
    let mut cur: * mut crate::src::list::_xmlList = 0 as *mut xmlList;
    if old.is_null() {
        return 0 as xmlListPtr;
    }
    cur = xmlListCreate(None, (*old).linkCompare);
    if cur.is_null() {
        return 0 as xmlListPtr;
    }
    if 0 as i32 != xmlListCopy(cur, old) {
        return 0 as xmlListPtr;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListCopy(
    mut cur: * mut crate::src::list::_xmlList,
    old: * mut crate::src::list::_xmlList,
) -> i32 {
    let mut lk: * mut crate::src::list::_xmlLink = 0 as *mut xmlLink;
    if old.is_null() || cur.is_null() {
        return 1 as i32;
    }
    lk = (*(*old).sentinel).next;
    while lk != (*old).sentinel {
        if 0 as i32 != xmlListInsert(cur, (*lk).data) {
            xmlListDelete(cur);
            return 1 as i32;
        }
        lk = (*lk).next;
    }
    return 0 as i32;
}
use crate::laertes_rt::*;
