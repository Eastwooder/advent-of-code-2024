<img src=".assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code {year}

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/) with a template based on [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust).

<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](../src/bin/01.rs) | `44.4µs` | `60.1µs` |
| [Day 2](../src/bin/02.rs) | `105.9µs` | `174.3µs` |
| [Day 3](../src/bin/03.rs) | `156.9µs` | `147.1µs` |
| [Day 4](../src/bin/04.rs) | `1.0ms` | `389.6µs` |
| [Day 5](../src/bin/05.rs) | `214.7µs` | `1.3ms` |
| [Day 6](../src/bin/06.rs) | `92.6µs` | `4.1ms` |
| [Day 7](../src/bin/07.rs) | `620.3µs` | `4.2ms` |
| [Day 8](../src/bin/08.rs) | `29.9µs` | `63.5µs` |
| [Day 9](../src/bin/09.rs) | `383.7µs` | `152.6ms` |
| [Day 10](../src/bin/10.rs) | `630.6µs` | `1.2ms` |
| [Day 11](../src/bin/11.rs) | `33.6µs` | `1.3ms` |
| [Day 12](../src/bin/12.rs) | `622.6µs` | `1.8ms` |
| [Day 13](../src/bin/13.rs) | `418.1µs` | `308.5µs` |
| [Day 14](../src/bin/14.rs) | `27.7µs` | `196.0ms` |
| [Day 16](../src/bin/16.rs) | `5.8ms` | `12.5ms` |
| [Day 18](../src/bin/18.rs) | `439.3µs` | `5.7ms` |
| [Day 19](../src/bin/19.rs) | `9.1ms` | `26.9ms` |
| [Day 22](../src/bin/22.rs) | `816.1µs` | `113.1ms` |
| [Day 23](../src/bin/23.rs) | `4.8ms` | `2.3ms` |

**Total: 549.48ms**
<!--- benchmarking table --->

My repository for the [2024 Edition of the Advent of Code](https://adventofcode.com/2024).

## Development

To get fast feedback just run `just check` and start coding.

---

## Toolchain

### Nix based

Just run `nix develop` to have all the required tools available. 
If you have `direnv` enabled then you will automatically switch to the development setup.

### 💻 Setup rust

Alternatively, you can install the toolchain yourself.

1.  Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2.  (recommended) Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) extension for your code editor.
3.  (optional) Install a native debugger. If you are using VS Code, [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a good option.

---

## Usage

If you are solving a previous year's advent of code, change the `AOC_YEAR` variable in `.cargo/config.toml` to reflect the year you are solving.

Additionally, the environment variable `SESSION_COOKIE` needs to be set to your advent of code's session cookie value, otherwise submissions and download of the input will not work!
(The local tests will be unaffected and can still be executed)

### ➡️ Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/template.txt) has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug your solutions against the example input. In VS Code, `rust-analyzer` will display buttons for running / debugging these unit tests above the unit test blocks.

> [!TIP]
> If a day has multiple example inputs, you can use the `read_file_part()` helper in your tests instead of `read_file()`. If this e.g. applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));`. This supports an arbitrary number of example files.

### ➡️ Download input for a day

> [!IMPORTANT] 
> Requires `SESSION_COOKIE` environment variable to be set.

You can automatically download puzzle input and description by either appending the `--download` flag to `scaffold` (e.g. `cargo scaffold 4 --download`) or with the separate `download` command:

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# ---
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

#### Submitting solutions

> [!IMPORTANT]
> Requires `SESSION_COOKIE` environment variable to be set.

Append the `--submit <part>` option to the `solve` command to submit your solution for checking.

### ➡️ Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

### ➡️ Benchmark your solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark your code and store timings in the readme. When benching, the runner will run your code between `10` and `10.000` times, depending on execution time of first execution, and print the average execution time.

`cargo time` has three modes of execution:

 1. `cargo time` without arguments incrementally benches solutions that do not have been stored in the readme yet and skips the rest.
 2. `cargo time <day>` benches a single solution.
 3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag: `cargo time --store`.

> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.

### ➡️ Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### ➡️ Read puzzle description

> [!IMPORTANT]
> Requires `SESSION_COOKIE` environment variable to be set.

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### ➡️ Scaffold, download & read the current aoc day

> [!IMPORTANT]
> Requires `SESSION_COOKIE` environment variable to be set.

During december, the `today` shorthand command can be used to:

 - scaffold a solution for the current day
 - download its input
 - and read the puzzle

in one go.

```sh
# example: `cargo today` on December 1st
cargo today

# output:
# Created module file "src/bin/01.rs"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# ---
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
#
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### ➡️ Format code

```sh
cargo fmt
```

### ➡️ Lint code

```sh
cargo clippy
```

## Optional template features

### Automatically track ⭐️ progress in the readme

This template includes [a GitHub action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`.
-   `AOC_YEAR`: the year you want to track. Example: `2021`.
-   `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

Go to the _Variables_ tab in your repository settings and create the following variable:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker. After you complete AoC or no longer work on it, you can set this to `false` to disable the CI.

✨ You can now run this action manually via the _Run workflow_ button on the workflow page. If you want the workflow to run automatically, uncomment the `schedule` section in the `readme-stars.yml` workflow file or add a `push` trigger.

### Enable code formatting / clippy checks in the CI

Uncomment the respective sections in the `ci.yml` workflow.

### Use DHAT to profile heap allocations

If you are not only interested in the runtime of your solution, but also its memory allocation profile, you can use the template's [DHAT](https://valgrind.org/docs/manual/dh-manual.html) integration to analyze it. In order to activate DHAT, call the `solve` command with the `--dhat` flag.

```sh
cargo solve 1 --dhat

# output:
#     Running `target/dhat/1`
# dhat: Total:     276 bytes in 3 blocks
# dhat: At t-gmax: 232 bytes in 2 blocks
# dhat: At t-end:  0 bytes in 0 blocks
# dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
# Part 1: 9001 (4.1ms)
```

The command will output some basic stats to the command-line and generate a `dhat-heap.json` report in the repo root directory.

You can pass the report a tool like [dh-view](https://nnethercote.github.io/dh_view/dh_view.html) to view a detailed breakdown of heap allocations.

### Use VS Code to debug your code

1.  Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
2.  Set breakpoints in your code. [^3]
3.  Click _Debug_ next to the unit test or the _main_ function. [^4]
4.  The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessred.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Common pitfalls

-   **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
[^3]:
    <img src="https://user-images.githubusercontent.com/1682504/198838369-453dc22c-c645-4803-afe0-fc50d5a3f00c.png" alt="Set a breakpoint" width="450" />

[^4]:
    <img alt="Run debugger" src="https://user-images.githubusercontent.com/1682504/198838372-c89369f6-0d05-462e-a4c7-8cd97b0912e6.png" width="450" />

[^5]:
    <img alt="Inspect debugger state" src="https://user-images.githubusercontent.com/1682504/198838373-36df6996-23bf-4757-9335-0bc4c1db0276.png" width="450" />
