use bimap::BiMap;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

type Dictionary = BiMap<String, usize>; // BiMap
type SentenceArray = Vec<Vec<usize>>;

pub fn normalise(data: &str) -> (Dictionary, SentenceArray) {
    // let stemmer = Stemmer::create(Algorithm::English);
    // let sw: HashSet<_> = Spark::stopwords(Language::English)
    //     .unwrap()
    //     .iter()
    //     .collect();
    // let unique_words: HashSet<String> = data
    //     .to_lowercase()
    //     .unicode_words()
    //     .map(|x| stemmer.stem(x))
    //     .filter(|x| !sw.contains(&x.deref()))
    //     .map(|y| y.to_string())
    //     .collect();
    let data = data.to_lowercase();
    let unique_words: BiMap<String, usize> = data
        .unicode_words()
        .into_iter()
        .unique()
        .enumerate()
        .map(|(idx, val)| (val.to_string(), idx))
        .collect();
    // HashMap<&str, usize> = a.unicode_words().into_iter().unique().enumerate().map(|(idx, val)| (val, idx)).collect();
    let sentences: Vec<Vec<usize>> = data
        .unicode_sentences()
        .map(|x| {
            x.unicode_words()
                .map(|y| unique_words.get_by_left(y).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    // TODO: turn the inner "sentences" into Compressed Vectors
    (unique_words, sentences)
}

// fn main() {
//     let stops: HashSet<_> = Spark::stopwords(Language::English)
//         .unwrap()
//         .iter()
//         .collect();
//     let mut tokens = vec!["brocolli", "is", "good", "to", "eat"];
//     tokens.retain(|s| !stops.contains(s));
//     assert_eq!(tokens, vec!("brocolli", "good", "eat"));
// }
