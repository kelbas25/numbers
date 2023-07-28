import random

INPUT_FILE = "num.in"
NUM_LINES = 10**6


def generate_input_file(file_path, num_lines):
    with open(file_path, 'w') as file:
        for _ in range(num_lines):
            rand_integer = random.randint(-(2**63), 2**63 - 1)
            file.write(f"{rand_integer}\n")


if __name__ == "__main__":
    generate_input_file(INPUT_FILE, NUM_LINES)
