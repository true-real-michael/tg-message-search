DICT_PATH = "data/dict.opcorpora.txt"
OUTPUT_PATH = "data/lemmatization-ru-ids.tsv"


if __name__ == "__main__":
    with open(DICT_PATH) as input_file:
        with open(OUTPUT_PATH, "w") as output_file:
            current_words = set()
            current_normal_form = ""
            for line in input_file:
                line = line.strip()
                if not line:
                    output_file.write(current_normal_form + '\t' + '\t'.join(current_words) + '\n')
                    current_words = set()
                    current_normal_form = ""
                    continue
                if line.isdecimal():
                    continue

                line = line.split()[0].lower()
                if not current_normal_form:
                    current_normal_form = line
                else:
                    current_words.add(line)
