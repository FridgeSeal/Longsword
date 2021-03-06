use crate::pipeline;
// use fst::{
//     automaton::{Str, Subsequence, Union},
//     map::Stream,
//     Automaton, IntoStreamer, Map, Streamer,
// };
use log::{error, info};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

type Dictionary = HashMap<String, usize>;
type SentenceArray = Vec<Vec<usize>>;
pub struct Document {
    pub name: String,
    pub dictionary: Dictionary,
    inverse_dict: HashMap<usize, String>,
    sentence_set: SentenceArray,
}

impl Document {
    pub fn lookup_word(&self, key: &str) -> Option<&usize> {
        self.dictionary.get(key)
    }

    pub fn new(name: impl Into<String>, data: String) -> Self {
        let word_len = data.len();
        info!("Prepping to index {} words", word_len);
        let (dictionary, sentences) = pipeline::normalise(&data);
        let reverse_dict = dictionary.iter().map(|(s, k)| (*k, s.clone())).collect();
        info!("Constructed Reverse Dictionary");
        Self {
            name: name.into(),
            dictionary: dictionary,
            sentence_set: sentences,
            inverse_dict: reverse_dict,
        }
        // let d1: Vec<(String, u64)> = data
        //     .iter()
        //     .zip(1..word_len)
        //     .map(|(x, y)| (x.to_string(), y as u64))
        //     .collect();
        // let d2 = d1.clone();
        // info!("Constructed word arrays");
        // let forward_dict = Map::from_iter(d1).expect("Failed to create forward dictionary");
        // info!("Constructed Forward Dictionary");
        // let reverse_dict = d2.iter().map(|(s, k)| (*k, s.clone())).collect();
        // info!("Constructed Reverse Dictionary");
        // Self {
        //     name: name.into(),
        //     dictionary: forward_dict,
        //     reverse_dictionary: reverse_dict,
        //     stemmer: Stemmer::create(Algorithm::English),
        //     stopwords: Spark::stopwords(Language::English)
        //         .expect("Failed to instantiate stopwords")
        //         .into_iter()
        //         .map(|&x| x.to_string())
        //         .collect(),
        // }
    }

    fn key_lookup(&self, id_array: &Vec<usize>) -> String {
        let words: Vec<String> = id_array
            .iter()
            .map(|idx| &self.inverse_dict[idx])
            .map(|x| x.to_owned())
            .collect();
        words.join(" ")
    }

    fn search_for_term(&self, s: &str) -> Vec<String> {
        info!("Preparing to perform search on terms: {}", s);
        if let Some(id) = self.dictionary.get(s) {
            info!("Found search term: ");
            let matching: Vec<String> = self
                .sentence_set
                .iter()
                .filter(|&x| x.contains(id))
                .map(|y| self.key_lookup(y))
                .collect();
            matching
        } else {
            vec![]
        }
    }

    pub fn search(&self, s: &str) -> Vec<String> {
        let s = s.to_lowercase();
        info!("Provided search data: {}", s);
        s.unicode_words()
            .map(|t| self.search_for_term(t))
            .filter(|x| x.len() > 0)
            .flatten()
            .collect()
    }

    //     let s2 = s
    //         .to_lowercase()
    //         .unicode_words()
    //         .map(|x| self.stemmer.stem(x))
    //         .map(|y| y.to_string())
    //         .filter(|x| !self.stopwords.contains(x))
    //         .collect::<Vec<String>>();
    //     info!("Search terms normalised: {:#?}", s2);
    //     let aut_set = s2
    //         .iter()
    //         .map(|x| Subsequence::new(&x).union(fst::automaton::Str::new(&x)))
    //         .collect::<Vec<Union<Subsequence, Str>>>();
    //     info!("Automaton created");
    //     let keys = aut_set
    //         .iter()
    //         .filter_map(|aut| {
    //             self.dictionary
    //                 .search(&aut)
    //                 .into_stream()
    //                 .into_str_keys()
    //                 .ok()
    //         })
    //         .flatten()
    //         .collect::<Vec<String>>();
    //     info!("Search performed!");
    //     keys
    // }
}
