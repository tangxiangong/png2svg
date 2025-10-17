# PNG2SVG

## 🚀 快速开始

### Python API

```python
from png2svg import png2svg, png2svgs

# 转换单个文件
png2svg("input.png")

# 转换单个文件并指定输出目录
png2svg("input.png", output_dir="output/")

# 递归转换目录中的所有 PNG 文件
png2svg("images/", directory=True, output_dir="svg_output/")

# 并行转换多个文件
png2svgs(["file1.png", "file2.png", "file3.png"], output_dir="output/")
```

### 命令行工具

```bash
# 转换单个文件
uv run main.py image.png

# 转换单个文件并指定输出目录
uv run main.py image.png -o output/

# 递归转换目录中的所有 PNG 文件
uv run main.py -d images/

# 并行转换多个文件
uv run main.py file1.png file2.png file3.png

# 查看帮助
uv run main.py -h
```

## 📚 API 文档

### `png2svg(filename, directory=False, output_dir=None)`

将 PNG 图像或目录转换为 SVG 格式。

**参数：**
- `filename` (str): PNG 文件的路径，或当 `directory=True` 时为目录路径
- `directory` (bool): 如果为 `True`，则递归转换指定目录中的所有 PNG 文件。默认为 `False`
- `output_dir` (Optional[str]): SVG 文件的输出目录。如果为 `None`，则输出到与输入相同的位置

### `png2svgs(filenames, output_dir=None)`

并行转换多个 PNG 图像为 SVG 格式。

**参数：**
- `filenames` (list[str]): 要转换的 PNG 文件路径列表
- `output_dir` (Optional[str]): SVG 文件的输出目录。如果为 `None`，则输出到与输入相同的位置

## 🔧 开发

### 环境要求

- Rust 1.85+
- Python 3.9+
- uv 或 pip

### 构建项目

```bash
# 安装依赖
uv sync

# 激活虚拟环境
# Linux / macOS
source .venv/bin/activate
# Windows
.venv\Scripts\activate

# 构建 Rust 扩展
maturin build --release
```

### 项目结构

```
png2svg/
├── png2svg-core/       # Rust 核心库
│   ├── src/
│   │   ├── convert.rs  # 转换算法
│   │   ├── error.rs    # 错误处理
│   │   └── lib.rs      # 库入口
│   └── examples/       # 示例代码
├── src/                # Python 绑定
│   ├── lib.rs          # PyO3 模块
│   └── ffi.rs          # FFI 函数
├── png2svg/            # Python 包
│   ├── __init__.py
│   ├── core.py         # Python API
│   └── _core.pyi       # 类型存根
├── main.py             # 命令行工具
└── pyproject.toml      # 项目配置
```

## 算法原理

PNG2SVG 使用以下算法将位图转换为矢量图：

1. **颜色分组**：使用 BFS（广度优先搜索）将相同颜色的连续像素分组
2. **边缘提取**：提取每个颜色区域的边缘
3. **路径连接**：将边缘连接成闭合路径，并优化直线段
4. **SVG 生成**：为每个颜色区域生成 SVG path 元素
