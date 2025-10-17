import os
from typing import Optional
from png2svg import _core


def png2svg(
    filename: str, directory: bool = False, output_dir: Optional[str] = None
) -> None:
    """
    将 PNG 图像或目录转换为 SVG 格式。

    Args:
        filename (str): PNG 文件的路径，或当 directory=True 时为目录路径。
        directory (bool): 如果为 True，则递归转换指定目录中的所有 PNG 文件。默认为 False。
        output_dir (Optional[str]): SVG 文件的输出目录。如果为 None，则输出到与输入相同的位置。

    Returns:
        None
    """
    if directory:
        return _core.convert_directory(filename, output_dir)
    if output_dir is not None:
        os.makedirs(output_dir, exist_ok=True)

    return _core.convert(filename, output_dir)


def png2svgs(filename: list[str], output_dir: Optional[str] = None) -> None:
    """
    并行转换多个 PNG 图像为 SVG 格式。

    Args:
        filename (list[str]): 要转换的 PNG 文件路径列表。
        output_dir (Optional[str]): SVG 文件的输出目录。如果为 None，则输出到与输入相同的位置。

    Returns:
        None
    """
    if output_dir is not None:
        os.makedirs(output_dir, exist_ok=True)

    return _core.convert_parallel(filename, output_dir)
