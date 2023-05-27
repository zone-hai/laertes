use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: i32,
        _: u64,
    ) -> *mut libc::c_void;
<<<<<<< HEAD
    
    
    
    
}
pub use crate::src::globals::__xmlGenericError;
pub use crate::src::globals::__xmlGenericErrorContext;
pub use crate::src::globals::xmlFree;
pub use crate::src::globals::xmlMalloc;
pub type size_t = crate::src::HTMLparser::size_t;
pub type xmlFreeFunc = crate::src::HTMLparser::xmlFreeFunc;
pub type xmlMallocFunc = crate::src::HTMLparser::xmlMallocFunc;
pub type xmlGenericErrorFunc = crate::src::HTMLparser::xmlGenericErrorFunc;
=======
    static mut xmlFree: xmlFreeFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    static mut xmlMalloc: xmlMallocFunc;
}
pub type size_t = u64;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLink {
    pub next: *mut _xmlLink,
    pub prev: *mut _xmlLink,
    pub data: *mut libc::c_void,
}
<<<<<<< HEAD
pub type xmlLink = crate::src::c14n::xmlLink;
pub type xmlLinkPtr = crate::src::c14n::xmlLinkPtr;
=======
pub type xmlLink = _xmlLink;
pub type xmlLinkPtr = *mut xmlLink;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlList {
    pub sentinel: xmlLinkPtr,
    pub linkDeallocator: Option::<unsafe extern "C" fn(xmlLinkPtr) -> ()>,
    pub linkCompare: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
    >,
}
<<<<<<< HEAD
pub type xmlList = crate::src::c14n::xmlList;
pub type xmlListPtr = crate::src::c14n::xmlListPtr;
pub type xmlListDeallocator = crate::src::c14n::xmlListDeallocator;
pub type xmlListDataCompare = crate::src::c14n::xmlListDataCompare;
pub type xmlListWalker = crate::src::c14n::xmlListWalker;
=======
pub type xmlList = _xmlList;
pub type xmlListPtr = *mut xmlList;
pub type xmlListDeallocator = Option::<unsafe extern "C" fn(xmlLinkPtr) -> ()>;
pub type xmlListDataCompare = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type xmlListWalker = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> i32,
>;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
unsafe extern "C" fn xmlLinkDeallocator(mut l: xmlListPtr, mut lk: xmlLinkPtr) {
    let ref mut fresh0 = (*(*lk).prev).next;
    *fresh0 = (*lk).next;
    let ref mut fresh1 = (*(*lk).next).prev;
    *fresh1 = (*lk).prev;
    if ((*l).linkDeallocator).is_some() {
        ((*l).linkDeallocator).expect("non-null function pointer")(lk);
    }
    xmlFree.expect("non-null function pointer")(lk as *mut libc::c_void);
}
<<<<<<< HEAD
 extern "C" fn xmlLinkCompare(
=======
unsafe extern "C" fn xmlLinkCompare(
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    mut data0: *const libc::c_void,
    mut data1: *const libc::c_void,
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut deallocator: xmlListDeallocator,
    mut compare: xmlListDataCompare,
) -> xmlListPtr {
    let mut l: xmlListPtr = 0 as *mut xmlList;
    l = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlList>() as u64) as xmlListPtr;
    if l.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
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
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
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
            xmlLinkCompare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> i32,
        );
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSearch(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 1 as i32;
    }
    lkPlace = xmlListLowerSearch(l, data);
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 1 as i32;
    }
    lkPlace = xmlListHigherSearch(l, data);
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
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
pub unsafe extern "C" fn xmlListDelete(mut l: xmlListPtr) {
    if l.is_null() {
        return;
    }
    xmlListClear(l);
    xmlFree.expect("non-null function pointer")((*l).sentinel as *mut libc::c_void);
    xmlFree.expect("non-null function pointer")(l as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveFirst(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
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
pub unsafe extern "C" fn xmlListClear(mut l: xmlListPtr) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return;
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        let mut next: xmlLinkPtr = (*lk).next;
        xmlLinkDeallocator(l, lk);
        lk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEmpty(mut l: xmlListPtr) -> i32 {
    if l.is_null() {
        return -(1 as i32);
    }
    return ((*(*l).sentinel).next == (*l).sentinel) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListFront(mut l: xmlListPtr) -> xmlLinkPtr {
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    return (*(*l).sentinel).next;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEnd(mut l: xmlListPtr) -> xmlLinkPtr {
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    return (*(*l).sentinel).prev;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSize(mut l: xmlListPtr) -> i32 {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut count: i32 = 0 as i32;
    if l.is_null() {
        return -(1 as i32);
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        lk = (*lk).next;
        count += 1;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopFront(mut l: xmlListPtr) {
    if xmlListEmpty(l) == 0 {
        xmlLinkDeallocator(l, (*(*l).sentinel).next);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopBack(mut l: xmlListPtr) {
    if xmlListEmpty(l) == 0 {
        xmlLinkDeallocator(l, (*(*l).sentinel).prev);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPushFront(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as i32;
    }
    lkPlace = (*l).sentinel;
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
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
pub unsafe extern "C" fn xmlListPushBack(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as i32;
    }
    lkPlace = (*(*l).sentinel).prev;
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlLink>() as u64) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
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
pub unsafe extern "C" fn xmlLinkGetData(mut lk: xmlLinkPtr) -> *mut libc::c_void {
    if lk.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*lk).data;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverse(mut l: xmlListPtr) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkPrev: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return;
    }
    lkPrev = (*l).sentinel;
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
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
pub unsafe extern "C" fn xmlListSort(mut l: xmlListPtr) {
    let mut lTemp: xmlListPtr = 0 as *mut xmlList;
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
    mut l: xmlListPtr,
    mut walker: xmlListWalker,
    mut user: *mut libc::c_void,
) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
pub unsafe extern "C" fn xmlListReverseWalk(
    mut l: xmlListPtr,
    mut walker: xmlListWalker,
    mut user: *mut libc::c_void,
) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() || walker.is_none() {
        return;
    }
    lk = (*(*l).sentinel).prev;
    while lk != (*l).sentinel {
        if walker.expect("non-null function pointer")((*lk).data, user)
            == 0 as i32
        {
            break;
        }
        lk = (*lk).prev;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListMerge(mut l1: xmlListPtr, mut l2: xmlListPtr) {
    xmlListCopy(l1, l2);
    xmlListClear(l2);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListDup(old: xmlListPtr) -> xmlListPtr {
    let mut cur: xmlListPtr = 0 as *mut xmlList;
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
    mut cur: xmlListPtr,
    old: xmlListPtr,
) -> i32 {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
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
