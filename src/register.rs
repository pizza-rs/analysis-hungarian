use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::stem::HungarianLightStemFilter;
use crate::stop::HungarianStopFilter;

pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("hungarian_light_stem", Box::new(HungarianLightStemFilter::new()));
    factory.register_token_filter("hungarian_stop", Box::new(HungarianStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(HungarianStopFilter::new()),
        Box::new(HungarianLightStemFilter::new()),
    ];

    factory.register_analyzer(
        "hungarian",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("hungarian_light_stem").is_some());
        assert!(factory.get_token_filter("hungarian_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("hungarian").is_some());
    }
}
