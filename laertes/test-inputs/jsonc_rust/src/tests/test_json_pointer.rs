use ::libc;
extern "C" {
    pub type json_object;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn json_object_put(obj: *mut json_object) -> libc::c_int;
    fn json_object_is_type(obj: *const json_object, type_0: json_type) -> libc::c_int;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_get(
        obj: *const json_object,
        key: *const libc::c_char,
    ) -> *mut json_object;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_array_get_idx(
        obj: *const json_object,
        idx: size_t,
    ) -> *mut json_object;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_get_int(obj: *const json_object) -> int32_t;
    fn json_object_new_string(s: *const libc::c_char) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const libc::c_char;
    fn json_object_equal(obj1: *mut json_object, obj2: *mut json_object) -> libc::c_int;
    fn json_pointer_get(
        obj: *mut json_object,
        path: *const libc::c_char,
        res: *mut *mut json_object,
    ) -> libc::c_int;
    fn json_pointer_getf(
        obj: *mut json_object,
        res: *mut *mut json_object,
        path_fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn json_pointer_set(
        obj: *mut *mut json_object,
        path: *const libc::c_char,
        value: *mut json_object,
    ) -> libc::c_int;
    fn json_pointer_setf(
        obj: *mut *mut json_object,
        value: *mut json_object,
        path_fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn json_tokener_parse(str: *const libc::c_char) -> *mut json_object;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type json_type = libc::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_pointer_map_s_i {
    pub s: *const libc::c_char,
    pub i: libc::c_int,
}
unsafe extern "C" fn test_example_int(
    mut jo1: *mut json_object,
    mut json_pointer: *const libc::c_char,
    mut expected_int: libc::c_int,
) {
    let mut jo2: *mut json_object = 0 as *mut json_object;
    if 0 as libc::c_int
        == json_pointer_get(jo1, json_pointer, 0 as *mut *mut json_object)
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, json_pointer, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            14 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void test_example_int(struct json_object *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int == json_pointer_get(jo1, json_pointer, &mut jo2) {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, json_pointer, &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            15 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void test_example_int(struct json_object *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    if json_object_is_type(jo2, json_type_int) != 0 {} else {
        __assert_fail(
            b"json_object_is_type(jo2, json_type_int)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            16 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void test_example_int(struct json_object *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    if expected_int == json_object_get_int(jo2) {} else {
        __assert_fail(
            b"expected_int == json_object_get_int(jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            17 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void test_example_int(struct json_object *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - GET -  %s == %d\n\0" as *const u8 as *const libc::c_char,
        json_pointer,
        expected_int,
    );
}
static mut input_json_str: *const libc::c_char = b"{ 'foo': ['bar', 'baz'], '': 0, 'a/b': 1, 'c%d': 2, 'e^f': 3, 'g|h': 4, 'i\\\\j': 5, 'k\\\"l': 6, ' ': 7, 'm~n': 8 }\0"
    as *const u8 as *const libc::c_char;
static mut rec_input_json_str: *const libc::c_char = b"{'arr' : [{'obj': [{},{},{'obj1': 0,'obj2': \"1\"}]}],'obj' : {'obj': {'obj': [{'obj1': 0,'obj2': \"1\"}]}}}\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn test_example_get() {
    let mut i: libc::c_int = 0;
    let mut jo1: *mut json_object = 0 as *mut json_object;
    let mut jo2: *mut json_object = 0 as *mut json_object;
    let mut jo3: *mut json_object = 0 as *mut json_object;
    let mut json_pointers: [json_pointer_map_s_i; 10] = [
        {
            let mut init = json_pointer_map_s_i {
                s: b"/\0" as *const u8 as *const libc::c_char,
                i: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/a~1b\0" as *const u8 as *const libc::c_char,
                i: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/c%d\0" as *const u8 as *const libc::c_char,
                i: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/e^f\0" as *const u8 as *const libc::c_char,
                i: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/g|h\0" as *const u8 as *const libc::c_char,
                i: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/i\\j\0" as *const u8 as *const libc::c_char,
                i: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/k\"l\0" as *const u8 as *const libc::c_char,
                i: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/ \0" as *const u8 as *const libc::c_char,
                i: 7 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: b"/m~0n\0" as *const u8 as *const libc::c_char,
                i: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = json_pointer_map_s_i {
                s: 0 as *const libc::c_char,
                i: 0 as libc::c_int,
            };
            init
        },
    ];
    jo1 = json_tokener_parse(input_json_str);
    if !jo1.is_null() {} else {
        __assert_fail(
            b"NULL != jo1\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - GET - LOADED TEST JSON\n\0" as *const u8 as *const libc::c_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, json_object_get_string(jo1));
    jo2 = 0 as *mut json_object;
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_get(jo1, b"\0" as *const u8 as *const libc::c_char, &mut jo2)
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if json_object_equal(jo2, jo1) != 0 {} else {
        __assert_fail(
            b"json_object_equal(jo2, jo1)\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - GET - ENTIRE OBJECT WORKED\n\0" as *const u8 as *const libc::c_char,
    );
    jo3 = json_object_new_array();
    json_object_array_add(
        jo3,
        json_object_new_string(b"bar\0" as *const u8 as *const libc::c_char),
    );
    json_object_array_add(
        jo3,
        json_object_new_string(b"baz\0" as *const u8 as *const libc::c_char),
    );
    jo2 = 0 as *mut json_object;
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"/foo\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/foo\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_get(jo1, b"/foo\0" as *const u8 as *const libc::c_char, &mut jo2)
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/foo\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if !jo2.is_null() {} else {
        __assert_fail(
            b"NULL != jo2\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if json_object_equal(jo2, jo3) != 0 {} else {
        __assert_fail(
            b"json_object_equal(jo2, jo3)\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    json_object_put(jo3);
    printf(
        b"PASSED - GET - /foo == ['bar', 'baz']\n\0" as *const u8 as *const libc::c_char,
    );
    jo2 = 0 as *mut json_object;
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"/foo/0\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/foo/0\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"/foo/0\0" as *const u8 as *const libc::c_char,
            &mut jo2,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/foo/0\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            116 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if !jo2.is_null() {} else {
        __assert_fail(
            b"NULL != jo2\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            b"bar\0" as *const u8 as *const libc::c_char,
            json_object_get_string(jo2),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"bar\", json_object_get_string(jo2))\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - GET - /foo/0 == 'bar'\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while !(json_pointers[i as usize].s).is_null() {
        test_example_int(jo1, json_pointers[i as usize].s, json_pointers[i as usize].i);
        i += 1;
    }
    json_object_put(jo1);
}
unsafe extern "C" fn test_recursion_get() {
    let mut jo2: *mut json_object = 0 as *mut json_object;
    let mut jo1: *mut json_object = json_tokener_parse(rec_input_json_str);
    jo2 = 0 as *mut json_object;
    if !jo1.is_null() {} else {
        __assert_fail(
            b"jo1 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, json_object_get_string(jo1));
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"/arr/0/obj/2/obj1\0" as *const u8 as *const libc::c_char,
            &mut jo2,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/arr/0/obj/2/obj1\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            135 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if json_object_is_type(jo2, json_type_int) != 0 {} else {
        __assert_fail(
            b"json_object_is_type(jo2, json_type_int)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            136 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int == json_object_get_int(jo2) {} else {
        __assert_fail(
            b"0 == json_object_get_int(jo2)\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            137 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"/arr/0/obj/2/obj2\0" as *const u8 as *const libc::c_char,
            &mut jo2,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/arr/0/obj/2/obj2\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            139 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if json_object_is_type(jo2, json_type_string) != 0 {} else {
        __assert_fail(
            b"json_object_is_type(jo2, json_type_string)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            b"1\0" as *const u8 as *const libc::c_char,
            json_object_get_string(jo2),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"1\", json_object_get_string(jo2))\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            141 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_getf(
            jo1,
            &mut jo2 as *mut *mut json_object,
            b"/%s/%d/%s/%d/%s\0" as *const u8 as *const libc::c_char,
            b"arr\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"obj\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            b"obj2\0" as *const u8 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_getf(jo1, &jo2, \"/%s/%d/%s/%d/%s\", \"arr\", 0, \"obj\", 2, \"obj2\")\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            143 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if json_object_is_type(jo2, json_type_string) != 0 {} else {
        __assert_fail(
            b"json_object_is_type(jo2, json_type_string)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            144 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            b"1\0" as *const u8 as *const libc::c_char,
            json_object_get_string(jo2),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"1\", json_object_get_string(jo2))\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if !jo1.is_null() {} else {
        __assert_fail(
            b"jo1 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            147 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"/obj/obj/obj/0/obj1\0" as *const u8 as *const libc::c_char,
            &mut jo2,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/obj/obj/obj/0/obj1\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if json_object_is_type(jo2, json_type_int) != 0 {} else {
        __assert_fail(
            b"json_object_is_type(jo2, json_type_int)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int == json_object_get_int(jo2) {} else {
        __assert_fail(
            b"0 == json_object_get_int(jo2)\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            150 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_get(
            jo1,
            b"/obj/obj/obj/0/obj2\0" as *const u8 as *const libc::c_char,
            &mut jo2,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_get(jo1, \"/obj/obj/obj/0/obj2\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            152 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if json_object_is_type(jo2, json_type_string) != 0 {} else {
        __assert_fail(
            b"json_object_is_type(jo2, json_type_string)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            153 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            b"1\0" as *const u8 as *const libc::c_char,
            json_object_get_string(jo2),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"1\", json_object_get_string(jo2))\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_getf(
            jo1,
            &mut jo2 as *mut *mut json_object,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"\0\0" as *const u8 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_getf(jo1, &jo2, \"%s\", \"\\0\")\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_recursion_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - GET - RECURSION TEST\n\0" as *const u8 as *const libc::c_char);
    json_object_put(jo1);
}
unsafe extern "C" fn test_wrong_inputs_get() {
    let mut jo2: *mut json_object = 0 as *mut json_object;
    let mut jo1: *mut json_object = json_tokener_parse(input_json_str);
    if !jo1.is_null() {} else {
        __assert_fail(
            b"NULL != jo1\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - GET - LOADED TEST JSON\n\0" as *const u8 as *const libc::c_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, json_object_get_string(jo1));
    jo2 = 0 as *mut json_object;
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"foo/bar\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"foo/bar\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"foo/bar\0" as *const u8 as *const libc::c_char,
            &mut jo2,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"foo/bar\", &jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            176 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if jo2.is_null() {} else {
        __assert_fail(
            b"jo2 == NULL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            177 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - GET - MISSING /\n\0" as *const u8 as *const libc::c_char);
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            0 as *mut json_object,
            b"foo/bar\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(NULL, \"foo/bar\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            182 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            0 as *mut json_object,
            0 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(NULL, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            185 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_getf(
            0 as *mut json_object,
            0 as *mut *mut json_object,
            0 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_getf(NULL, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            188 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(jo1, 0 as *const libc::c_char, 0 as *mut *mut json_object)
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            192 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_getf(jo1, 0 as *mut *mut json_object, 0 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 != json_pointer_getf(jo1, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - GET - NULL INPUTS\n\0" as *const u8 as *const libc::c_char);
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"/foo/a\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"/foo/a\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            200 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            201 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"/foo/01\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"/foo/01\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            204 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_getf(
            jo1,
            0 as *mut *mut json_object,
            b"/%s/a\0" as *const u8 as *const libc::c_char,
            b"foo\0" as *const u8 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_getf(jo1, NULL, \"/%s/a\", \"foo\")\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            206 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"/foo/-\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"/foo/-\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            209 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"/foo/4\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"/foo/4\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            213 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 2 as libc::c_int {} else {
        __assert_fail(
            b"errno == ENOENT\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            214 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_getf(
            jo1,
            0 as *mut *mut json_object,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"/foo/22\0" as *const u8 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_getf(jo1, NULL, \"%s\", \"/foo/22\")\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 2 as libc::c_int {} else {
        __assert_fail(
            b"errno == ENOENT\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_getf(
            jo1,
            0 as *mut *mut json_object,
            b"/%s/%d\0" as *const u8 as *const libc::c_char,
            b"foo\0" as *const u8 as *const libc::c_char,
            22 as libc::c_int,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_getf(jo1, NULL, \"/%s/%d\", \"foo\", 22)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 2 as libc::c_int {} else {
        __assert_fail(
            b"errno == ENOENT\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"/foo/-1\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"/foo/-1\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            223 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 22 as libc::c_int {} else {
        __assert_fail(
            b"errno == EINVAL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    *__errno_location() = 0 as libc::c_int;
    if 0 as libc::c_int
        != json_pointer_get(
            jo1,
            b"/foo/10\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_get(jo1, \"/foo/10\", NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 2 as libc::c_int {} else {
        __assert_fail(
            b"errno == ENOENT\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            227 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_get(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - GET - INVALID INDEXES\n\0" as *const u8 as *const libc::c_char);
    json_object_put(jo1);
}
unsafe extern "C" fn test_example_set() {
    let mut jo2: *mut json_object = 0 as *mut json_object;
    let mut jo1: *mut json_object = json_tokener_parse(input_json_str);
    if !jo1.is_null() {} else {
        __assert_fail(
            b"jo1 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            237 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - SET - LOADED TEST JSON\n\0" as *const u8 as *const libc::c_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, json_object_get_string(jo1));
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/foo/1\0" as *const u8 as *const libc::c_char,
            json_object_new_string(b"cod\0" as *const u8 as *const libc::c_char),
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/foo/1\", json_object_new_string(\"cod\"))\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            241 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == strcmp(
            b"cod\0" as *const u8 as *const libc::c_char,
            json_object_get_string(
                json_object_array_get_idx(
                    json_object_object_get(
                        jo1,
                        b"foo\0" as *const u8 as *const libc::c_char,
                    ),
                    1 as libc::c_int as size_t,
                ),
            ),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"cod\", json_object_get_string(json_object_array_get_idx( json_object_object_get(jo1, \"foo\"), 1)))\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            243 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - SET - 'cod' in /foo/1\n\0" as *const u8 as *const libc::c_char);
    jo2 = json_tokener_parse(b"[1,2,3]\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int
        != json_pointer_set(
            &mut jo1,
            b"/fud/gaw\0" as *const u8 as *const libc::c_char,
            jo2,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(&jo1, \"/fud/gaw\", (jo2 = json_tokener_parse(\"[1,2,3]\")))\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            245 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    if *__errno_location() == 2 as libc::c_int {} else {
        __assert_fail(
            b"errno == ENOENT\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            246 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - non-existing /fud/gaw\n\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/fud\0" as *const u8 as *const libc::c_char,
            json_object_new_object(),
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/fud\", json_object_new_object())\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            248 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - SET - /fud == {}\n\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/fud/gaw\0" as *const u8 as *const libc::c_char,
            jo2,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/fud/gaw\", jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            250 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - /fug/gaw == [1,2,3]\n\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/fud/gaw/0\0" as *const u8 as *const libc::c_char,
            json_object_new_int(0 as libc::c_int),
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/fud/gaw/0\", json_object_new_int(0))\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            252 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_setf(
            &mut jo1 as *mut *mut json_object,
            json_object_new_int(0 as libc::c_int),
            b"%s%s/%d\0" as *const u8 as *const libc::c_char,
            b"/fud\0" as *const u8 as *const libc::c_char,
            b"/gaw\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_setf(&jo1, json_object_new_int(0), \"%s%s/%d\", \"/fud\", \"/gaw\", 0)\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            253 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - /fug/gaw == [0,2,3]\n\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/fud/gaw/-\0" as *const u8 as *const libc::c_char,
            json_object_new_int(4 as libc::c_int),
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/fud/gaw/-\", json_object_new_int(4))\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            255 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - /fug/gaw == [0,2,3,4]\n\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/\0" as *const u8 as *const libc::c_char,
            json_object_new_int(9 as libc::c_int),
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/\", json_object_new_int(9))\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            257 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - SET - / == 9\n\0" as *const u8 as *const libc::c_char);
    jo2 = json_tokener_parse(
        b"{ 'foo': [ 'bar', 'cod' ], '': 9, 'a/b': 1, 'c%d': 2, 'e^f': 3, 'g|h': 4, 'i\\\\j': 5, 'k\\\"l': 6, ' ': 7, 'm~n': 8, 'fud': { 'gaw': [ 0, 2, 3, 4 ] } }\0"
            as *const u8 as *const libc::c_char,
    );
    if json_object_equal(jo2, jo1) != 0 {} else {
        __assert_fail(
            b"json_object_equal(jo2, jo1)\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - Final JSON is: %s\n\0" as *const u8 as *const libc::c_char,
        json_object_get_string(jo1),
    );
    json_object_put(jo2);
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"\0" as *const u8 as *const libc::c_char,
            json_object_new_int(10 as libc::c_int),
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"\", json_object_new_int(10))\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            267 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    if 10 as libc::c_int == json_object_get_int(jo1) {} else {
        __assert_fail(
            b"10 == json_object_get_int(jo1)\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            268 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_example_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, json_object_get_string(jo1));
    json_object_put(jo1);
}
unsafe extern "C" fn test_wrong_inputs_set() {
    let mut jo2: *mut json_object = 0 as *mut json_object;
    let mut jo1: *mut json_object = json_tokener_parse(input_json_str);
    if !jo1.is_null() {} else {
        __assert_fail(
            b"jo1 != NULL\0" as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            278 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(b"PASSED - SET - LOADED TEST JSON\n\0" as *const u8 as *const libc::c_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, json_object_get_string(jo1));
    if 0 as libc::c_int
        != json_pointer_set(
            0 as *mut *mut json_object,
            0 as *const libc::c_char,
            0 as *mut json_object,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(NULL, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            282 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        != json_pointer_setf(
            0 as *mut *mut json_object,
            0 as *mut json_object,
            0 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_setf(NULL, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            283 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        != json_pointer_set(&mut jo1, 0 as *const libc::c_char, 0 as *mut json_object)
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(&jo1, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            284 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        != json_pointer_setf(
            &mut jo1 as *mut *mut json_object,
            0 as *mut json_object,
            0 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_setf(&jo1, NULL, NULL)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            285 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - failed with NULL params for input json & path\n\0" as *const u8
            as *const libc::c_char,
    );
    jo2 = json_object_new_string(b"cod\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int
        != json_pointer_set(
            &mut jo1,
            b"foo/bar\0" as *const u8 as *const libc::c_char,
            jo2,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(&jo1, \"foo/bar\", (jo2 = json_object_new_string(\"cod\")))\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            288 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - failed 'cod' with path 'foo/bar'\n\0" as *const u8
            as *const libc::c_char,
    );
    json_object_put(jo2);
    jo2 = json_object_new_string(b"cod\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int
        != json_pointer_setf(
            &mut jo1 as *mut *mut json_object,
            jo2,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"foo/bar\0" as *const u8 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_setf(&jo1, (jo2 = json_object_new_string(\"cod\")), \"%s\", \"foo/bar\")\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - failed 'cod' with path 'foo/bar'\n\0" as *const u8
            as *const libc::c_char,
    );
    json_object_put(jo2);
    jo2 = json_object_new_string(b"cod\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int
        != json_pointer_set(&mut jo1, b"0\0" as *const u8 as *const libc::c_char, jo2)
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(&jo1, \"0\", (jo2 = json_object_new_string(\"cod\")))\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            297 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    printf(
        b"PASSED - SET - failed with invalid array index'\n\0" as *const u8
            as *const libc::c_char,
    );
    json_object_put(jo2);
    jo2 = json_object_new_string(b"whatever\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int
        != json_pointer_set(
            &mut jo1,
            b"/fud/gaw\0" as *const u8 as *const libc::c_char,
            jo2,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(&jo1, \"/fud/gaw\", jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            302 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/fud\0" as *const u8 as *const libc::c_char,
            json_object_new_object(),
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/fud\", json_object_new_object())\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            303 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    if 0 as libc::c_int
        == json_pointer_set(
            &mut jo1,
            b"/fud/gaw\0" as *const u8 as *const libc::c_char,
            jo2,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_set(&jo1, \"/fud/gaw\", jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            304 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    jo2 = json_object_new_int(0 as libc::c_int);
    if 0 as libc::c_int
        != json_pointer_set(
            &mut jo1,
            b"/fud/gaw/0\0" as *const u8 as *const libc::c_char,
            jo2,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(&jo1, \"/fud/gaw/0\", jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            308 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    json_object_put(jo2);
    jo2 = json_object_new_int(0 as libc::c_int);
    if 0 as libc::c_int
        != json_pointer_set(
            &mut jo1,
            b"/fud/gaw/\0" as *const u8 as *const libc::c_char,
            jo2,
        )
    {} else {
        __assert_fail(
            b"0 != json_pointer_set(&jo1, \"/fud/gaw/\", jo2)\0" as *const u8
                as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            311 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    json_object_put(jo2);
    printf(
        b"PASSED - SET - failed to set index to non-array\n\0" as *const u8
            as *const libc::c_char,
    );
    if 0 as libc::c_int
        == json_pointer_setf(
            &mut jo1 as *mut *mut json_object,
            json_object_new_string(b"cod\0" as *const u8 as *const libc::c_char),
            b"%s\0" as *const u8 as *const libc::c_char,
            b"\0\0" as *const u8 as *const libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 == json_pointer_setf(&jo1, json_object_new_string(\"cod\"), \"%s\", \"\\0\")\0"
                as *const u8 as *const libc::c_char,
            b"/home/xial/json-c/tests/test_json_pointer.c\0" as *const u8
                as *const libc::c_char,
            315 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_wrong_inputs_set(void)\0"))
                .as_ptr(),
        );
    }
    json_object_put(jo1);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    test_example_get();
    test_recursion_get();
    test_wrong_inputs_get();
    test_example_set();
    test_wrong_inputs_set();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
