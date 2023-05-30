unqual_name: &str

type_sig: TypeSig
enum TypeSig {
    /// An unknown type (coming from type definitions)
    Unknown,
    /// A struct with field names.
    Struct(Vec<String>),
    /// A C union with field names
    Union(Vec<String>),
}

def_name: &String

type_defs: HashMap<String, HashMap<TypeSig, String>>

pub struct json_object {
    pub o_type: json_type,
    pub _ref_count: uint32_t,
    pub _to_json_string: Option::<json_object_to_json_string_fn>,
    pub _pb: *mut printbuf,
    pub _user_delete: Option::<json_object_delete_fn>,
    pub _userdata: *mut libc::c_void,
}
unqual_name : json_object
type_sig : < o_type, _ref_count, _to_json_string, _pb, _user_delete, _userdata >
def_name : src::json_object::json_object