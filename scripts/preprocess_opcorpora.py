import pickle

DICT_PATH = "data/dict.opcorpora.txt"
OUTPUT_PATH = "data/lemmatization-ru.tsv"


if __name__ == "__main__":
    with open(DICT_PATH) as input_file:
        with open(OUTPUT_PATH, "w") as output_file:
            current_normal_form = ""
            for line in input_file:
                line = line.strip()
                if not line:
                    continue
                if line.isdecimal():
                    current_normal_form = ""
                    continue

                line = line.split()[0].lower()
                if not current_normal_form:
                    current_normal_form = line
                else:
                    file.write(f"{line}\t{current_normal_form}\n")
