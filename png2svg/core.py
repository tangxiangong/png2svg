from png2svg import _core


def png2svg(filename: str, directory: bool = False) -> None:
    """
    Convert a PNG image to SVG format.

    Args:
        filename (str): The path to the PNG file to be converted.

    Returns:
        None
    """
    if directory:
        return _core.convert_directory(filename)
    return _core.convert(filename)


def png2svgs(filename: list[str]) -> None:
    """
    Convert a PNG image to SVG format.

    Args:
        filename (str): The path to the PNG file to be converted.

    Returns:
        None
    """

    return _core.convert_parallel(filename)
