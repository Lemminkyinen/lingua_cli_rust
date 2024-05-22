import zlib
from pathlib import Path
from typing import Union


def decompress_zlib(
    input_file: Union[str, Path], output_file: Union[str, Path]
) -> None:
    with open(input_file, "rb") as f_in:
        compressed_data = f_in.read()

    decompressed_data = zlib.decompress(compressed_data)

    with open(output_file, "wb") as f_out:
        f_out.write(decompressed_data)


def main():
    decompress_zlib("files/dictionary.json.zlib", "files/dictionary.json")


if __name__ == "__main__":
    main()
