import os
from typing import Optional, Union

from . import _core

__all__ = ["convert"]


def convert(
    filename: Union[str, list[str]],
    directory: bool = False,
    output_dir: Optional[str] = None,
) -> None:
    """
    将 PNG 图像或目录转换为 SVG 格式。

    Args:
        filename (str/list[str]): PNG 文件的路径，或当 directory=True 时为目录路径。
        directory (bool): 如果为 True，则递归转换指定目录中的所有 PNG 文件。默认为 False。
        output_dir (Optional[str]): SVG 文件的输出目录。如果为 None，则输出到与输入相同的位置。

    Returns:
        None
    """
    if output_dir is not None:
        os.makedirs(output_dir, exist_ok=True)

    if isinstance(filename, list):
        _core.convert_parallel(filename, output_dir)
    if isinstance(filename, str):
        if directory:
            return _core.convert_directory(filename, output_dir)
        else:
            return _core.convert(filename, output_dir)

    raise TypeError("filename 参数必须是字符串或字符串列表")
