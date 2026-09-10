
import sys

if len(sys.argv) != 3:
    print(f"Usage: {sys.argv[0]} <keyword> <file_path>")
    sys.exit(1)

keyword = sys.argv[1]
file_path = sys.argv[2]

with open(file_path, "r") as file:
    words = file.read().split()

found = False

# Linear search
for i, word in enumerate(words):
    if word.lower() == keyword.lower():
        print(f"Found '{keyword}' at position {i + 1}")
        found = True
        break

if not found:
    print(f"'{keyword}' not found")
