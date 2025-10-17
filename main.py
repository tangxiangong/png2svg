# type: ignore
# pyright: reportAny = false
import argparse
import sys
from pathlib import Path

from png2svg import png2svg, png2svgs


def main():
    parser = argparse.ArgumentParser(
        description="将 PNG 图像转换为 SVG 格式",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
使用示例:
  # 转换单个 PNG 文件
  python main.py image.png

  # 转换单个 PNG 文件并指定输出目录
  python main.py image.png -o output/

  # 递归转换目录中的所有 PNG 文件
  python main.py -d images/

  # 并行转换多个 PNG 文件
  python main.py file1.png file2.png file3.png
        """,
    )

    _ = parser.add_argument(
        "input",
        nargs="*",
        help="要转换的 PNG 文件，或使用 -d/--directory 时指定的目录",
    )

    _ = parser.add_argument(
        "-d",
        "--directory",
        action="store_true",
        help="递归转换指定目录中的所有 PNG 文件",
    )

    _ = parser.add_argument(
        "-o",
        "--output",
        type=str,
        default=None,
        help="SVG 文件的输出目录（默认：与输入相同）",
    )

    args = parser.parse_args()

    # 验证输入
    if not args.input:
        parser.error("请指定至少一个输入文件或目录")

    try:
        if args.directory:
            # 目录转换模式
            if len(args.input) != 1:
                parser.error("使用 -d/--directory 时请指定一个目录")

            directory = args.input[0]
            if not Path(directory).is_dir():
                print(f"错误: '{directory}' 不是有效的目录", file=sys.stderr)
                return 1

            print(f"正在递归转换 '{directory}' 中的所有 PNG 文件...")
            png2svg(directory, directory=True, output_dir=args.output)
            print("✓ 目录转换完成！")

        elif len(args.input) == 1:
            # 单文件模式
            filename = args.input[0]
            if not Path(filename).exists():
                print(f"错误: 文件 '{filename}' 不存在", file=sys.stderr)
                return 1

            print(f"正在转换 '{filename}'...")
            png2svg(filename, output_dir=args.output)
            print("✓ 转换完成！")

        else:
            # 多文件模式（并行）
            for filename in args.input:
                if not Path(filename).exists():
                    print(f"错误: 文件 '{filename}' 不存在", file=sys.stderr)
                    return 1

            print(f"正在并行转换 {len(args.input)} 个文件...")
            png2svgs(args.input, output_dir=args.output)
            print("✓ 所有转换完成！")

        return 0

    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
