<div align="center">

# 🇭🇺 pizza-analysis-hungarian

**Hungarian text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--hungarian-blue)](https://github.com/pizza-rs/analysis-hungarian)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Hungarian language analysis with a light stemmer and stop words. Hungarian is an
agglutinative language with extensive suffixation — the light stemmer provides
conservative reduction without over-stemming compound forms.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `hungarian_light_stem` | Hungarian light stemmer |
| TokenFilter | `hungarian_stop` | Hungarian stop words (198 entries) |
| Analyzer | `hungarian` | Full pipeline: lowercase → light_stem → stop |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_hungarian::register_all(&mut factory);

let analyzer = factory.get_analyzer("hungarian").unwrap();
// "házakat" (houses, accusative) → "ház"
```

## Installation

```toml
[dependencies]
pizza-analysis-hungarian = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["hungarian"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
