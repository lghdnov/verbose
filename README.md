# verbose

A command-line utility that converts numbers into their spoken word representations.

## Installation

```bash
cargo install --git https://github.com/lghdnov/verbose
```

## Usage

```bash
verbose <number> --lang <language>
```

### Examples

```bash
# English
verbose 1234567 --lang en
# Output: one million two hundred thirty four thousand five hundred sixty seven

# Russian
verbose 1234567 --lang ru
# Output: один миллион двести тридцать четыре тысячи пятьсот шестьдесят семь

# Swedish
verbose 1234567 --lang sv
# Output: en miljon tvåhundratrettiofyra tusen femhundrasextiosju
```

## Supported Languages

- `en` - English
- `ru` - Russian
- `sv` - Swedish
