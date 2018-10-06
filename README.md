[![Build][build-img]][build-url]

# human

> Converts a numeric litaral into an human-readable format.

## Installation

`cargo build --release` will build the project. The program can be found under `target/release/human`.

`cargo run` will directly run the program.

## Usage

The program `human` will wait until you enter a numeric literal (and hit enter):
```shell
./human
123456
one hundred twenty three thousand, four hundred fifty six
```

### Caution

This program only works (in a useful way) until 999 vigintillion (999 * 10^63). Anything greater than that is expressed as `toolargeillion`. It also uses the short scale as commonly found in the (modern) English language.

[build-img]: https://travis-ci.com/flyingP0tat0/human.svg?branch=master
[build-url]: https://travis-ci.com/flyingP0tat0/human
