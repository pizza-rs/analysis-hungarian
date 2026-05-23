# pizza-analysis-hungarian

Hungarian language analysis with 4-step light stemmer and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `hungarian_stem` | Token Filter | Hungarian 4-step light stemmer: case → possessive → plural → vowel normalization (á→a, é→e) |
| `hungarian_stop` | Token Filter | Hungarian stop words filter (198 words) |
| `hungarian` | Analyzer | Full pipeline: lowercase → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "hungarian"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["hungarian_stem", "hungarian_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
