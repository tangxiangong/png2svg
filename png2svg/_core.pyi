def convert(_filename: str, _output_dir: str | None) -> None:
    """Convert a single PNG file to SVG format."""
    ...

def convert_parallel(_filenames: list[str], _output_dir: str | None) -> None:
    """Convert multiple PNG files to SVG format in parallel."""
    ...

def convert_directory(_directory: str, _output_dir: str | None) -> None:
    """Convert all PNG files in a directory to SVG format."""
    ...
