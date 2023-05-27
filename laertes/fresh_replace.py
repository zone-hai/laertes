# 这个脚本替换掉c2rust生成的fresh中间变量

import re

filename = './test-inputs/libxml2_rust/src/legacy.rs'

import re

# 读取文件数据
with open(filename, 'r') as f:
    data = f.read()

# 获取最大数字（即 fresh${num} 中最大的 ${num}）
max_num = max(map(int, re.findall(r'fresh(\d+)', data)))

# 定义正则表达式
pattern = r'let ref mut fresh(\d+)\s*=\s*(.*?);'

# 定义全局变量
exprs = [""] * (max_num + 1)

# 定义替换函数
def replace(match):
    num = int(match.group(1))
    expr = match.group(2)
    exprs[num] = expr
    return ""

# 替换匹配项
new_data = re.sub(pattern, replace, data)

# 处理 *fresh${num} 替换
lines = new_data.split("\n")  # 按行分割文本
new_lines = []
for i, line in enumerate(lines):
    for num in re.findall(r'fresh(\d+)', line):
        target = "*fresh" + num
        line = line.replace(target, "(" + exprs[int(num)] + ")")
        exprs[int(num)] = ""  # 清空记录的右侧表达式
    new_lines.append(line)

# 重新组装文本
new_data = "\n".join(new_lines)

# 将新数据写回文件
with open(filename, 'w') as f:
    f.write(new_data)