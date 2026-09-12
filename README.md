# Boyer–Moore Search

A small Rust command-line program that searches a text file for a pattern using the Boyer–Moore string-search technique.

The same family of fast substring-search techniques is used in tools such as
`grep`. Depending on the implementation and pattern, `grep` can use
Boyer–Moore or related algorithms to skip portions of the input instead of
checking every character sequentially.

## About

The Boyer–Moore algorithm compares a pattern with the text from right to left. When a mismatch occurs, it uses information from the pattern to skip text positions that cannot produce a match.

This implementation uses the bad-character shift table. For a pattern

$$P = p_0p_1\dots p_{m-1},$$

the table stores a shift value for each character. For an occurrence of character $c$ at position $i$, the shift is

$$\text{shift}(c) = \max(1, m - i - 1).$$

The rightmost occurrence of a character determines its final shift because later occurrences overwrite earlier entries in the table. If a mismatch occurs while comparing the text character $t$ with the pattern character $p_j$, the search advances by the shift associated with the mismatching pattern character:

$$s \leftarrow s + \text{shift}(p_j).$$

Here, $s$ is the current alignment of the pattern against the text. Matching proceeds from $p_{m-1}$ toward $p_0$. Once every character matches, the line is reported and the search moves to the next line.

## Complexity

For a pattern of length $m$ and text of length $n$:

- Preprocessing the shift table takes $O(m)$ time.
- The algorithm uses $O(\lvert\Sigma\rvert)$ additional space in the worst case, where $\Sigma$ is the set of characters in the table.
- The average search time is approximately $O(n/m)$ for favorable character distributions, because each mismatch can skip multiple text positions.
- The worst-case search time is $O(nm)$.

The program searches each line independently and reports only the first match per line. Line numbers are 1-based.

## Project Plan

The current implementation searches the file for the keyword sequentially using the Bad Character Heuristics to skip upon character mismatches. Goal of the project is to divide the file into safe segments and perform search on parallel threads:

1. Determine safe boundaries for splitting the file, preserving complete lines.
2. Assign each section to a worker thread.
3. Build or share the pattern shift table for each worker.
4. Run the Boyer–Moore search on each section concurrently.
5. Combine the results and restore their original file order.

If a pattern crosses a section boundary, neighboring sections will need a
small overlap of up to $m - 1$ characters, where $m$ is the pattern length.
This prevents matches from being lost during parallel processing.

Also, the current algorithm has high overhead and processes string. Subsequent versions will aim to process the chars as bytes and further performance optimizations.

## Installation

Install Rust and Cargo from [rustup](https://rustup.rs/), then clone or download this repository. Build the project with:

```bash
cargo build
```

Important: For an optimized build:

```bash
cargo build --release
```

## Usage

Run the program with a pattern and the path to a UTF-8 text file:

```bash
cargo run -- <pattern> <file-path>
```

Example:

```bash
cargo run -- "needle" ./examples/sample.txt
```

Matching lines are printed in the following format:

```text
Line 3: The needle appears on this line.
```

To run the optimized binary directly:

```bash
./target/release/BoyerMoore <pattern> <file-path>
```

The input file must be readable and encoded as UTF-8.
