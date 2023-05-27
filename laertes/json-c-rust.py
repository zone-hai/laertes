import os
import re

def replace_text(dir_path, old_text, new_text):
    for root, dirs, files in os.walk(dir_path):
        for file in files:
            if os.path.splitext(file)[1] == '.rs':
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                content = re.sub(old_text, new_text, content)
                with open(file_path, 'w') as f:
                    f.write(content)

def main():
    cwd = os.getcwd()
    work_dir = cwd + '/rewrite-workspace/jsonc_rust'
    rewrite_invocation = cwd + '/rewrite-workspace/jsonc_rust/lib.rs -L all=' + cwd + '/rewrite-workspace/jsonc_rust/target/debug/deps'
    with open("rewrite-invocations/jsonc_rust", "w") as file:
        file.write(rewrite_invocation)

    replace_text(work_dir, 'libc::c_char', 'i8')
    replace_text(work_dir, 'libc::c_uchar', 'u8')
    replace_text(work_dir, 'libc::c_short', 'i16')
    replace_text(work_dir, 'libc::c_ushort', 'u16')
    replace_text(work_dir, 'libc::c_int', 'i32')
    replace_text(work_dir, 'libc::c_uint', 'u32')
    replace_text(work_dir, 'libc::c_longlong', 'i64')
    replace_text(work_dir, 'libc::c_ulonglong', 'u64')
    replace_text(work_dir, 'libc::c_long', 'i64')
    replace_text(work_dir, 'libc::c_ulong', 'u64')
    replace_text(work_dir, 'libc::c_schar', 'i8')
    replace_text(work_dir, 'libc::c_double', 'f64')

    # with open('src/refslice.rs', 'r') as f:
    #     content = f.read()

    # with open(work_dir + '/lib.rs', 'a') as f:
    #     f.write(content)


if __name__ == "__main__":
    main()