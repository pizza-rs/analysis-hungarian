use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Hungarian light stemmer. Removes common case suffixes and plural markers.
/// Hungarian is agglutinative with extensive suffixation.
#[derive(Clone, Debug, Default)]
pub struct HungarianLightStemFilter;

impl HungarianLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for HungarianLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }

        let stemmed = stem_hungarian(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_hungarian(word: &str) -> String {
    let mut s = word.to_string();

    // Step 1: Remove case endings
    s = remove_case(&s);

    // Step 2: Remove possessive suffixes
    s = remove_possessive(&s);

    // Step 3: Remove plural markers
    s = remove_plural(&s);

    // Step 4: Normalize accented vowels
    s = normalize_hungarian(&s);

    s
}

fn remove_case(word: &str) -> String {
    let suffixes: &[&str] = &[
        // Long case forms
        "okban", "ekben", "ökben",
        // Standard case endings
        "nak", "nek", "ban", "ben", "ból", "ből",
        "hoz", "hez", "höz", "ról", "ről", "tól", "től",
        "val", "vel", "ért", "nál", "nél",
        "kor", "ból", "ből", "ról", "ről",
        "ba", "be", "ra", "re", "ul", "ül",
        "on", "en", "ön", "ig",
        "t", "n",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 4 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

fn remove_possessive(word: &str) -> String {
    let suffixes: &[&str] = &[
        "jainak", "jeinek",
        "jaink", "jeink",
        "jait", "jeit",
        "jaim", "jeid", "jain", "jein",
        "juk", "jük",
        "aink", "eink",
        "aim", "eid", "ain", "ein",
        "ait", "eit",
        "unk", "ünk",
        "ja", "je",
        "uk", "ük",
        "om", "em", "öm",
        "od", "ed", "öd",
        "m", "d",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 4 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

fn remove_plural(word: &str) -> String {
    let suffixes: &[&str] = &[
        "okat", "eket", "öket",
        "ok", "ek", "ök", "ak",
        "ák", "ék",
        "k",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 4 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

fn normalize_hungarian(word: &str) -> String {
    let mut result = String::with_capacity(word.len());
    let mut changed = false;
    for c in word.chars() {
        match c {
            'á' => { result.push('a'); changed = true; }
            'é' => { result.push('e'); changed = true; }
            _ => result.push(c),
        }
    }
    if changed { result } else { word.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_plural() {
        let filter = HungarianLightStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("házakat"),
            start_offset: 0,
            end_offset: 8,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        // "akat" removed → "ház"
        assert!(token.term.as_ref().starts_with("ház"));
    }
}
