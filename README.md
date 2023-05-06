# Scol (Stream Colorizer)

Scol is a simple command-line program that reads lines from standard input and colorizes them based on a regular expression pattern. The program supports setting both foreground and background colors for the matched text, using command-line arguments.

## Installation

To install the program, you'll need Rust and Cargo installed on your system. Then, run the following command:

```
cargo install scol
```

## Usage

The program's basic usage is:

```
scol <PATTERN> [--fg <COLOR>] [--bg <COLOR>] [--ignore-case] [--only-matching]
```

where `<PATTERN>` is a regular expression pattern that will be used to match lines, `--fg <COLOR>` sets the foreground color of the matched text (using one of the available colors: black, red, green, yellow, blue, cyan, white, magenta), `--bg <COLOR>` sets the background color of the matched text, and `--ignore-case` makes the pattern match case-insensitively.

At least one of `--fg` or `--bg` must be specified.

Here's an example command that colorizes all lines containing the word "error" in red text on a yellow background:

```
tail -f /var/log/syslog | scol error --fg red --bg yellow
```

## Example

Suppose you have a log file that contains lines like this:

```
[2022-05-06 10:23:45] INFO: Connected to database
[2022-05-06 10:24:13] WARNING: Disk space running low
[2022-05-06 10:25:01] ERROR: Database connection lost
[2022-05-06 10:26:19] INFO: System shutting down
```

You can use Scol to highlight lines containing the word "error" in red text on a yellow background, and lines containing the word "warning" in yellow text on a red background, using the following commands:

```
cat logfile.txt | scol error --fg red --bg yellow | scol warning --fg yellow --bg red
```

The output would be:

```
[2022-05-06 10:23:45] INFO: Connected to database
[2022-05-06 10:24:13] WARNING: Disk space running low
[2022-05-06 10:25:01] ERROR: Database connection lost
[2022-05-06 10:26:19] INFO: System shutting down
```

where the "warning" line is highlighted in yellow on a red background, and the "error" line is highlighted in red on a yellow background.

## License

This program is distributed under the MIT license. See the `LICENSE` file for details.
