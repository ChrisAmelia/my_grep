## MiniGrep

Learning project, built a mini `grep`.

## Build

```sh
cargo build --release
```

The executable will be present in `target/release`.

## How to use

```sh
minigrep [-flags] [word] [file1] [file2] [file3] ...
```

Note that the files are optional, if none are provided then `minigrep` looks in all files in the current working directory.
For now, minigrep only supports `-i/--insensitive` flag.

## Testing

Run the tests with:

```sh
cargo test
```
