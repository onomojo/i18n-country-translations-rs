# i18n-country-translations

[![Crates.io](https://img.shields.io/crates/v/i18n-country-translations.svg)](https://crates.io/crates/i18n-country-translations)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Localized country names for Rust -- 168 locales, 257 territory codes, zero runtime I/O.

Building a country picker? Displaying addresses internationally? Your users expect to see country names in their own language -- not just in English. Most i18n packages cover 30-50 locales and call it a day, leaving hundreds of millions of speakers without support.

**i18n-country-translations** provides country name translations sourced from [CLDR](https://cldr.unicode.org/), the same data that powers ICU, Chrome, and Android. With **168 locales** -- more than double the coverage of alternatives -- you can serve users from Amharic to Zulu without gaps. All translation data is embedded at compile time via `include_str!` -- no external files, no filesystem access, no network calls.

## Why i18n-country-translations?

- **168 locales** -- the most comprehensive coverage available on crates.io
- **257 territory codes** -- full ISO 3166-1 alpha-2 plus EU, XK, and other commonly used codes
- **CLDR-sourced** -- accurate, professionally reviewed translations (not scraped from Wikipedia)
- **Compile-time embedded** -- all data baked into the binary, zero runtime I/O
- **Two APIs** -- owned `Registry` struct for testability, or global convenience functions for simplicity
- **Thread-safe globals** -- global API is protected by `Mutex`, safe for concurrent use
- **Auto-uppercase** -- country codes are automatically uppercased, so `"us"` and `"US"` both work
- **No panic** -- all lookups return `Option`, never panics on missing data

## Install

```toml
[dependencies]
i18n-country-translations = "0.1"
```

## Quick Start

```rust
use i18n_country_translations::Registry;

let mut reg = Registry::new();
reg.register_locale("de").unwrap();
reg.set_default_locale("de").unwrap();

assert_eq!(reg.get_name("US"), Some("Vereinigte Staaten"));
```

## Usage

### Struct-based API (recommended)

The `Registry` struct gives you an owned, testable translation store:

```rust
use i18n_country_translations::Registry;

let mut reg = Registry::new();
reg.register_locale("de").unwrap();
reg.register_locale("ja").unwrap();
reg.set_default_locale("de").unwrap();

// Lookup with default locale
let name = reg.get_name("US");
assert_eq!(name, Some("Vereinigte Staaten"));

// Lookup with explicit locale
let name = reg.get_name_for_locale("ja", "JP");
assert_eq!(name, Some("日本"));

// Case-insensitive codes
let name = reg.get_name("us");
assert_eq!(name, Some("Vereinigte Staaten"));

// Norway works correctly (not parsed as boolean)
let name = reg.get_name("NO");
assert_eq!(name, Some("Norwegen"));

// List what's loaded
let locales = reg.registered_locales(); // ["de", "ja"]
```

### Register all locales at once

```rust
let mut reg = Registry::new();
reg.register_all_locales().unwrap();
// All 168 locales now available
```

### Global convenience API

For simpler use cases where you don't want to pass a `Registry` around:

```rust
i18n_country_translations::register_locale("en").unwrap();
i18n_country_translations::set_default_locale("en").unwrap();

let name = i18n_country_translations::get_name("NO");
assert_eq!(name, Some("Norway".to_string()));

// Explicit locale
let name = i18n_country_translations::get_name_for_locale("de", "US");
assert_eq!(name, Some("Vereinigte Staaten".to_string()));
```

### List available locales

```rust
// All locales in the embedded data (no registration needed)
let available = i18n_country_translations::available_locales();
assert!(available.len() >= 168);
assert!(available.contains(&"en"));
```

## API Reference

### `Registry` (struct-based)

| Method | Description |
|--------|-------------|
| `Registry::new()` | Create a new empty registry. |
| `register_locale(&mut self, locale)` | Load translations for a single locale. No-op if already registered. |
| `register_all_locales(&mut self)` | Load all 168 available locales. |
| `set_default_locale(&mut self, locale)` | Set the default locale. Returns `Err` if not registered. |
| `default_locale(&self)` | Get the current default locale. |
| `get_name(&self, code)` | Get the localized country name using the default locale. Code auto-uppercased. |
| `get_name_for_locale(&self, locale, code)` | Get the localized country name for a specific locale. Code auto-uppercased. |
| `available_locales()` | List all locales in the embedded data (static). |
| `registered_locales(&self)` | List all currently loaded locales. |

### Global functions

| Function | Description |
|----------|-------------|
| `register_locale(locale)` | Register a locale in the global registry. |
| `register_all_locales()` | Register all locales in the global registry. |
| `set_default_locale(locale)` | Set the global default locale. |
| `get_name(code)` | Look up using the global default locale. Returns `Option<String>`. Code auto-uppercased. |
| `get_name_for_locale(locale, code)` | Look up for a specific locale. Returns `Option<String>`. Code auto-uppercased. |
| `available_locales()` | List all available locales. |

All lookups return `None` when a code or locale is not found -- no panics.

## Supported Locales

168 locales covering major and regional languages worldwide:

<details>
<summary>View all 168 locales</summary>

af, ak, am, ar, as, az, be, bg, bm, bn, bo, br, bs, ca, cs, cy, da, de, dz, ee, el, en, eo, es, et, eu, fa, ff, fi, fo, fr, ga, gd, gl, gu, ha, he, hi, hr, hu, hy, ia, id, ig, is, it, ja, ka, ki, kk, kl, km, kn, ko, ky, lg, ln, lo, lt, lu, lv, mg, mk, ml, mn, mr, ms, mt, my, nb, nd, ne, nl, nn, or, pa, pl, ps, pt, pt-BR, rm, rn, ro, ru, se, sg, si, sk, sl, sn, so, sq, sr, sv, sw, ta, te, th, ti, to, tr, uk, ur, uz, vi, yo, zh, zh-CN, zh-HK, zh-TW, zu, asa, bas, bez, brx, byn, cgg, chr, dav, dje, dyo, ebu, ewo, fil, fur, gsw, guz, haw, jmc, kab, kam, kde, kea, khq, kln, ksb, ksf, ksh, lag, luo, luy, mas, mer, mfe, mgh, mua, naq, nmg, nus, nyn, rof, rwk, saq, sbp, seh, ses, shi, swc, teo, tig, twq, tzm, vai, vun, wae, wal, xog, yav

</details>

## Data Source

All translations come from the [Unicode CLDR](https://cldr.unicode.org/) (Common Locale Data Repository) -- the industry-standard source used by every major platform including iOS, Android, Chrome, and Java. This ensures translations are accurate, consistent, and maintained by native speakers through Unicode's established review process.

## Also Available For

- **[Ruby](https://github.com/onomojo/i18n-country-translations)** -- Rails gem with automatic Railtie integration
- **[JavaScript/TypeScript](https://github.com/onomojo/i18n-country-translations-js)** -- NPM package with tree-shaking and reverse lookups
- **[Go](https://github.com/onomojo/i18n-country-translations-go)** -- Go module with embedded data via `go:embed`

## License

MIT
