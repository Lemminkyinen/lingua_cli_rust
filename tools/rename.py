import concurrent.futures
from pathlib import Path


def rename_file(file: Path):
    if file.name.endswith("_.mp3"):
        new_name = file.name.replace("_.mp3", ".mp3")
        file.rename(file.with_name(new_name))

def main():
    path = Path("files/tones2")

    with concurrent.futures.ProcessPoolExecutor() as executor:
        for file in path.iterdir():
            if file.is_file():
                executor.submit(rename_file, file)

if __name__ == "__main__":
    main()