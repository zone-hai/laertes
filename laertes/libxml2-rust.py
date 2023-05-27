import os
import re

pattern_1 = r"let\s+ref\s+mut\s+(\w+)\s*=\s*([\s\S]*?);"
pattern_2 = r"let\s+ref\s+(\w+)\s*=\s*(.*);"
pattern_3 = r"\s+0\s+as\s+\*mut"
pattern_4 = r"(\s+\d\s+as\s+\*const\s+\[.*?\])"

def replace_text(dir_path, old_text, new_text):
    for root, dirs, files in os.walk(dir_path):
        for file in files:
            if os.path.splitext(file)[1] == '.rs':
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                result_1 = re.sub(old_text, new_text, content)
                with open(file_path, 'w') as f:
                    f.write(result_1)

def replace_pattern_1(dir_path):
    for root, dirs, files in os.walk(dir_path):
        for file in files:
            if os.path.splitext(file)[1] == '.rs':
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                result_2 = re.sub(pattern_1, r"let \1 = &mut (\2);", content)
                with open(file_path, 'w') as f:
                    f.write(result_2)

def replace_pattern_2(dir_path):
    for root, dirs, files in os.walk(dir_path):
        for file in files:
            if os.path.splitext(file)[1] == '.rs':
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                result_3 = re.sub(pattern_2, r"let \1 = &(\2);", content)
                with open(file_path, 'w') as f:
                    f.write(result_3)

def replace_pattern_3(dir_path):
    for root, dirs, files in os.walk(dir_path):
        for file in files:
            if os.path.splitext(file)[1] == '.rs':
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                # pattern = r'0\s+as\s+\*mut\s+(\w+)'
                # replace = r'0 as *mut libc::c_void as *mut \1'
                # pattern = r'0\s+as\s+\*mut\s+((?<!libc::c_void )\w+)'
                pattern = r'(?<!&)\b0\s+as\s+\*mut\s+((?:(?!libc::c_void|\*mut|\*const|i8|u8|i16|u16|i32|u32|i64|u64|f32|f64)[a-zA-Z0-9_])+)'
                replace = r'0 as *mut libc::c_void as *mut \1'
                result_3 = re.sub(pattern, replace, content)
                with open(file_path, 'w') as f:
                    f.write(result_3)

def replace_pattern_4(dir_path):
    for root, dirs, files in os.walk(dir_path):
        for file in files:
            if os.path.splitext(file)[1] == '.rs':
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                pattern = r'(?<!&)\b0\s+as\s+\*const\s+((?:(?!libc::c_void|\*mut|\*const|i8|u8|i16|u16|i32|u32|i64|u64|f32|f64)[a-zA-Z0-9_])+)'
                replace = r'0 as *const libc::c_void as *const \1'
                result_4 = re.sub(pattern, replace, content)
                with open(file_path, 'w') as f:
                    f.write(result_4)


def main():
    cwd = os.getcwd()
    work_dir = cwd + '/rewrite-workspace/libxml2_rust'
    rewrite_invocation = cwd + '/rewrite-workspace/libxml2_rust/lib.rs -L all=' + cwd + '/rewrite-workspace/libxml2_rust/target/debug/deps'
    with open("rewrite-invocations/libxml2_rust", "w") as file:
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
    replace_text(work_dir, 'libc::c_float', 'f32')


    # replace_pattern_1(work_dir)
    # replace_pattern_2(work_dir)
    # replace_pattern_3(work_dir)
    # replace_pattern_4(work_dir)


if __name__ == "__main__":
    main()