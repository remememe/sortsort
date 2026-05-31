# sortsort
terminal application written in Rust that visualizes sorting algorithms.

![Sorting](https://raw.githubusercontent.com/remememe/sortsort/refs/heads/main/assets/sorting.gif)

# Installation

### Binaries

Pre-build binaries [releases](https://github.com/remememe/sortsort/releases)

### With cargo

`cargo install sortsort`

# Usage

```
Usage: sortsort [OPTIONS]

Options:
  -a, --amount <AMOUNT>                Number of elements to sort [default: 4]
  -b, --border                         Enable border
  -c, --color <BORDER_COLOR>           Border color in RGB format R,G,B [default: 185,99,100]
  -l, --looped                         Loop the program
  -i, --info                           Display sorting statistics
  -s, --algorithm <SORTING_ALGORITHM>  Sorting method [default: Random] [possible values: quick, bubble, cocktail, selection, odd-even, gnome, insertion, shell, bogo, random]
      --width <BAR_WIDTH>              Width of each bar
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

# Contribution

Contributions are welcome! If you'd like to help improve the app whether by fixing bugs, adding new features, or optimizing performance feel free to:

- Open an issue if you find a bug or have a suggestion
- Submit a pull request on GitHub
