import json


def main():
    with open("files/dictionary.json") as file:
        data = json.load(file)

    for item in data:
        item["pinyin"] = item["pinyin"].lower()

    with open("files/dictionary.json", "w") as f:
        json.dump(data, f, ensure_ascii=False, indent=4)


if __name__ == "__main__":
    main()
