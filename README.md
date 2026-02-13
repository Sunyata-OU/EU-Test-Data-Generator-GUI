# EU Test Data Generator — Desktop GUI

Desktop application for generating valid, checksum-correct IBANs and personal ID codes. Built with [egui](https://github.com/emilk/egui), powered by [eu-test-data-generator](https://github.com/Sunyata-OU/EU-Test-Data-Generator).

## Features

- **IBAN tab** — Generate IBANs for 96 countries, pick a specific country or random
- **Personal ID tab** — Generate personal IDs for 31 countries with gender and birth year filters
- **Copy all** — One-click copy results to clipboard
- **Single binary** — No runtime dependencies, works offline

## Installation

### Download

Grab the latest binary for your OS from [Releases](https://github.com/Sunyata-OU/EU-Test-Data-GUI/releases).

### Build from source

```bash
git clone https://github.com/Sunyata-OU/EU-Test-Data-GUI.git
cd EU-Test-Data-GUI
cargo build --release
# Binary at target/release/eu-test-data-gui
```

#### Linux dependencies

On Debian/Ubuntu:
```bash
sudo apt install libgtk-3-dev libxdo-dev libatspi2.0-dev
```

On Arch:
```bash
sudo pacman -S gtk3 xdotool at-spi2-core
```

#### WSL2

If running under WSL2, use software rendering:
```bash
LIBGL_ALWAYS_SOFTWARE=1 ./eu-test-data-gui
```

## Usage

The app has two tabs:

**IBAN** — Select a country (or "Random"), set the count, and click Generate. Results show the formatted IBAN and validation status.

**Personal ID** — Select a country, set count, optionally filter by gender and birth year, then click Generate. Results show the code, gender, date of birth, and validation status.

Click **Copy all** to copy all generated codes to your clipboard.

## Supported Formats

- **96 IBAN countries** with correct BBAN national checksums
- **31 personal ID formats** including PESEL, Personnummer, Codice Fiscale, JMBG, Isikukood, HETU, and more

See the [core library](https://github.com/Sunyata-OU/EU-Test-Data-Generator) for the full list.

## License

MIT
