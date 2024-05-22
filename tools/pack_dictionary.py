import zlib
from pathlib import Path
from typing import Union


def compress_to_zlib(
    input_file: Union[str, Path], output_file: Union[str, Path]
) -> None:
    with open(input_file, "rb") as f_in:
        data = f_in.read()

    compressed_data = zlib.compress(data)

    with open(output_file, "wb") as f_out:
        f_out.write(compressed_data)


def main():
    compress_to_zlib("files/dictionary.json", "files/dictionary.json.zlib")


if __name__ == "__main__":
    main()
