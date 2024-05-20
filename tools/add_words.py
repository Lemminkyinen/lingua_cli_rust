import json
import sys
from typing import Any


def append_word(
    traditional: list[str], simplified: list[str], english: list[str], notes: str
):
    with open("files/words.json", "r") as f:
        words: list[dict[str, Any]] = json.load(f)

    words.append({
        "traditional": traditional,
        "simplified": simplified,
        "english": english,
        "notes": notes,
    })

    with open("files/words.json", "w") as f:
        json.dump(words, f, ensure_ascii=False, indent=4)


def main():
    try:
        while True:
            traditional = input("Enter the traditional word: ")
            english = input("Enter the English translation: ")

            traditional = traditional.split(",")
            traditional = list(map(str.strip, traditional))

            english = english.split(",")
            english = list(map(str.strip, english))

            append_word(traditional, traditional, english, None)
    except KeyboardInterrupt:
        sys.exit(0)


if __name__ == "__main__":
    main()
