//! Hungarian stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Hungarian stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "a",
    "abban",
    "ahhoz",
    "ahogy",
    "ahol",
    "aki",
    "akik",
    "akkor",
    "alatt",
    "amely",
    "amelyek",
    "amelyekben",
    "amelyeket",
    "amelyet",
    "amelynek",
    "ami",
    "amikor",
    "amit",
    "amolyan",
    "amíg",
    "annak",
    "arra",
    "arról",
    "az",
    "azok",
    "azon",
    "azonban",
    "azt",
    "aztán",
    "azután",
    "azzal",
    "azért",
    "be",
    "belül",
    "benne",
    "bár",
    "cikk",
    "cikkek",
    "cikkeket",
    "csak",
    "de",
    "e",
    "ebben",
    "eddig",
    "egy",
    "egyes",
    "egyetlen",
    "egyik",
    "egyre",
    "egyéb",
    "egész",
    "ehhez",
    "ekkor",
    "el",
    "ellen",
    "első",
    "elég",
    "elő",
    "először",
    "előtt",
    "emilyen",
    "ennek",
    "erre",
    "ez",
    "ezek",
    "ezen",
    "ezt",
    "ezzel",
    "ezért",
    "fel",
    "felé",
    "hanem",
    "hiszen",
    "hogy",
    "hogyan",
    "igen",
    "ill",
    "ill.",
    "illetve",
    "ilyen",
    "ilyenkor",
    "ismét",
    "ison",
    "itt",
    "jobban",
    "jó",
    "jól",
    "kell",
    "kellett",
    "keressünk",
    "keresztül",
    "ki",
    "kívül",
    "között",
    "közül",
    "legalább",
    "legyen",
    "lehet",
    "lehetett",
    "lenne",
    "lenni",
    "lesz",
    "lett",
    "maga",
    "magát",
    "majd",
    "meg",
    "mellett",
    "mely",
    "melyek",
    "mert",
    "mi",
    "mikor",
    "milyen",
    "minden",
    "mindenki",
    "mindent",
    "mindig",
    "mint",
    "mintha",
    "mit",
    "mivel",
    "miért",
    "most",
    "már",
    "más",
    "másik",
    "még",
    "míg",
    "nagy",
    "nagyobb",
    "nagyon",
    "ne",
    "nekem",
    "neki",
    "nem",
    "nincs",
    "néha",
    "néhány",
    "nélkül",
    "olyan",
    "ott",
    "pedig",
    "persze",
    "rá",
    "s",
    "saját",
    "sem",
    "semmi",
    "sok",
    "sokat",
    "sokkal",
    "szemben",
    "szerint",
    "szinte",
    "számára",
    "talán",
    "tehát",
    "teljes",
    "tovább",
    "továbbá",
    "több",
    "ugyanis",
    "utolsó",
    "után",
    "utána",
    "vagy",
    "vagyis",
    "vagyok",
    "valaki",
    "valami",
    "valamint",
    "való",
    "van",
    "vannak",
    "vele",
    "vissza",
    "viszont",
    "volna",
    "volt",
    "voltak",
    "voltam",
    "voltunk",
    "által",
    "általában",
    "át",
    "én",
    "éppen",
    "és",
    "így",
    "össze",
    "úgy",
    "új",
    "újabb",
    "újra",
    "ő",
    "ők",
    "őket",
    ];
    words.iter().copied().collect()
});

/// Removes Hungarian stop words from the token stream.
#[derive(Clone, Debug)]
pub struct HungarianStopFilter {
    stop_words: HashSet<String>,
}

impl Default for HungarianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl HungarianStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for HungarianStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 198);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = HungarianStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = HungarianStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = HungarianStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
