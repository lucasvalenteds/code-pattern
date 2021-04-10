# Code Pattern

It formats a simple string according to a pattern informed by the user.

## Examples

```
$ ./code-pattern
Usage: ./code-pattern <delimiter> <mask> <raw_code>
Example: ./code-pattern - xx-xx-xxxx 12052005
```

```
$ ./code-pattern / xx/xx/xxxx 04102021
04/10/2021
```

```
$ ./code-pattern - xx-xx-xxxx 04102021
04-10-2021
```

```
$ ./code-pattern __ x__xx__xxx__xxxx aBCdefGHIJ
a__BC__def__GHIJ
```

## How to run

| Description | Command |
| :--- | :--- |
| Run tests | `cargo test` |
| Run for development | `cargo run` |
| Generate the binary | `cargo build --release` |
| Format source code | `cargo fmt` |

