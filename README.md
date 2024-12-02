# AdventOfCode 2024

Stack:
- Rust

## Layout

- [`assignments/`](./assignments/) - the Puzzle Assignments from AdventOfCode.com
- [`input/`](./input/) - the data-file associated with each puzzle
- [`src/puzzle/`](./src/puzzle/) - the source code for any given puzzle
-	[`src/prelude.rs`](./src/prelude.rs) - automatically import default types/crates into every puzzle
- [`src/main.rs`](./src/main.rs) [`src/lib.rs`](./src/lib.rs) - CLI Command & Subcommands
- [`src/puzzle.rs`](./src/puzzle.rs) - Type every daily puzzle conforms to

## Usage

- `cargo run -- -d1 -p1 download`
- `cargo run -- -d1 -p1 puzzle`
- `cargo watch -x "run -- -d01 -p1 puzzle"`
- `cargo run -- -d1 -p1 puzzle --submit`

### Authentication

- Go to AdventOfCode, login to your account.
- Open DevTools, find the Storage->Cookies
- Find the `session` cookie, and copy its value
- `pbpaste > ~/.adventofcode.session`
