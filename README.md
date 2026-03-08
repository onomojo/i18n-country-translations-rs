# i18n-country-translations-rs

Rust crate for localized country name translations. Covers 257 territory codes across 168 locales, sourced from Unicode CLDR.

All translation data is embedded at compile time — zero runtime I/O, no external files needed.

## Install

```toml
[dependencies]
i18n-country-translations = "0.1"
```

## Usage

```rust
use i18n_country_translations::Registry;

let mut reg = Registry::new();
reg.register_locale("de").unwrap();
reg.set_default_locale("de").unwrap();

assert_eq!(reg.get_name("US"), Some("Vereinigte Staaten"));
```

### Global API

```rust
i18n_country_translations::register_locale("en").unwrap();
i18n_country_translations::set_default_locale("en").unwrap();
let name = i18n_country_translations::get_name("NO"); // Some("Norway")
```

## License

MIT
