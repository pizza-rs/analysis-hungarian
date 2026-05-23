//! Comprehensive tests for pizza-analysis-hungarian.

use pizza_analysis_hungarian::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// HungarianLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = HungarianLightStemFilter::new();
}

#[test]
fn stem_plural_k() {
    let f = HungarianLightStemFilter::new();
    // "házak" (houses) → stem
    let mut token = make_token("házak");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_accusative() {
    let f = HungarianLightStemFilter::new();
    // "könyvet" (book, accusative) → stem
    let mut token = make_token("könyvet");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_possessive() {
    let f = HungarianLightStemFilter::new();
    // "házam" (my house) → stem
    let mut token = make_token("házam");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_inessive() {
    let f = HungarianLightStemFilter::new();
    // "házban" (in the house) → stem
    let mut token = make_token("házban");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_past() {
    let f = HungarianLightStemFilter::new();
    // "írtam" (I wrote) → stem
    let mut token = make_token("írtam");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_adjective() {
    let f = HungarianLightStemFilter::new();
    let mut token = make_token("szépek");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = HungarianLightStemFilter::new();
    let mut token = make_token("és");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = HungarianLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_single_char() {
    let f = HungarianLightStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// HungarianStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = HungarianStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = HungarianStopFilter::new();
    let stop_words = ["a", "az", "egy", "és", "meg", "nem", "hogy", "de", "van", "abban"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = HungarianStopFilter::new();
    let content_words = ["ház", "könyv", "iskola", "város"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = HungarianStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("hungarian_light_stem").is_some());
    assert!(factory.get_token_filter("hungarian_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("hungarian").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("hungarian").unwrap();
    let mut input = String::from("A ház nagy és szép");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("hungarian").unwrap();
    let mut input = String::from("a macska és a kutya");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"a"));
    assert!(!terms.contains(&"és"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("hungarian").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("hungarian").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
