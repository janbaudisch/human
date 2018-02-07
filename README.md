# human
converts numeric litaral into humand-readable format

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

Please note that leading zeros are not allowed.
