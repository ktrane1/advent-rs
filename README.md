Practing Rust with 2022 AOC challenges. I'll attempt the 2023 AOC challenges as they are released and update this repo - time and ability
willing :)

Note: `<DAY>` should evaluate to a number. ie. `1`, `2`, `3` etc.

Create module for new day:
```bash
cargo run init <DAY>
```

This will create a directory, files as well as update the main function with imports
and case match.

Run with `testdata`:

```bash
cargo run exec <DAY> --test
```

Run with `data`:

```bash
cargo run exec <DAY>
```

