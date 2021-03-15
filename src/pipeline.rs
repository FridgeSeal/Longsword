use bimap::BiMap;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

type Dictionary = BiMap<String, usize>; // BiMap
type SentenceArray = Vec<Vec<usize>>;

pub fn normalise(data: &str) -> (Dictionary, SentenceArray) {
    let data = data.to_lowercase();
    let unique_words: BiMap<String, usize> = data
        .unicode_words()
        .into_iter()
        .unique()
        .enumerate()
        .map(|(idx, val)| (val.to_string(), idx))
        .collect();
    let sentences: Vec<Vec<usize>> = data
        .unicode_sentences()
        .map(|x| {
            x.unicode_words()
                .map(|y| unique_words.get_by_left(y).unwrap().to_owned())
                .collect::<Vec<usize>>()
        })
        .collect();
    // TODO: turn the inner "sentences" into Compressed Vectors
    (unique_words, sentences)
}
