use crate::pipeline;
use bimap::BiMap;
use cuckoofilter::{self, CuckooFilter};
use itertools::Itertools;
use log::info;
use rkyv::{Archive, Deserialize, Serialize};
use slab::Slab;
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Display, Formatter},
};
use unicode_segmentation::UnicodeSegmentation;

type Dictionary = BiMap<String, usize>;
type SentenceArray = Vec<Vec<usize>>;

#[derive(Debug)]
pub struct SearchResult<'a> {
    pub doc_name: &'a str,
    pub n_hits: usize,
    // pub results: &'a [&'a String],
    pub results: Vec<String>,
}

impl Display for SearchResult<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Document name: {}\nNumber of occurrences: {}\nSamples: {:#?}",
            self.doc_name, self.n_hits, self.results
        )
    }
}

pub struct Index {
    name: String,
    // config: ????
    // statistics: Polars df
    p_filter: CuckooFilter<DefaultHasher>,
    pub documents: Slab<Document>,
    pub keys: Vec<usize>,
}
pub struct Document {
    pub name: String,
    pub dictionary: Dictionary,
    pub p_filter: CuckooFilter<DefaultHasher>,
    pub sentence_set: SentenceArray,
}

#[derive(Archive, Debug, Serialize, Deserialize)]
pub struct TextData {
    pub id: u64,
    pub name: String,
    pub text: SentenceArray,
}

// Want to add the bloom filter functionaity as some kind of trait? So we can write it once and just "attach"
// the functionality to each one in a nice, composable manner
//

impl Document {
    pub fn new(name: impl Into<String>, data: String) -> Self {
        let (dictionary, sentences) = pipeline::normalise(&data);
        let mut p_filter = CuckooFilter::new();
        dictionary
            .iter()
            .for_each(|(x, _)| p_filter.add(x).unwrap());
        Self {
            name: name.into(),
            dictionary,
            p_filter,
            sentence_set: sentences,
        }
    }

    fn key_lookup(&self, id_array: &Vec<usize>) -> String {
        let words: Vec<String> = id_array
            .iter()
            .map(|idx| self.dictionary.get_by_right(idx).unwrap())
            .map(|x| x.to_owned())
            .collect();
        words.join(" ")
    }

    fn search_for_term(&self, s: &str) -> Vec<String> {
        if let Some(id) = self.dictionary.get_by_left(s) {
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

    pub fn search(&self, s: &[&str]) -> Vec<String> {
        info!("Preparing to perform search on terms: {:?}", s);
        s.iter()
            .map(|t| self.search_for_term(t))
            .filter(|x| x.len() > 0)
            .flatten()
            .collect()
    }
}

impl Index {
    pub fn new(name: impl Into<String>, data: Vec<Document>) -> Self {
        let name = name.into();
        let mut documents = Slab::with_capacity(data.len() * 2);
        let keys: Vec<usize> = data.into_iter().map(|f| documents.insert(f)).collect();
        let mut p_filter = CuckooFilter::with_capacity(800_000);
        documents
            .iter()
            .map(|(_, f)| f.dictionary.left_values())
            .flatten()
            .for_each(|g| {
                let cuckoo_insert = p_filter.add(g);
                match cuckoo_insert {
                    Ok(_) => (),
                    Err(_) => log::warn!(
                        "Reached max capacity in Cuckoo filter, something is being ejected"
                    ),
                }
            });
        Self {
            name,
            p_filter,
            documents,
            keys,
        }
    }

    fn pre_filter(&self, terms: &[&str]) -> bool {
        terms.iter().any(|&f| self.p_filter.contains(f))
    }

    pub fn search(&self, search_term: &str) -> anyhow::Result<Vec<SearchResult>> {
        let s_lower = search_term.to_lowercase();
        let s = s_lower.unicode_words().unique().collect::<Vec<&str>>();
        let results = self
            .keys
            .iter()
            .map(|k| self.documents.get(*k).unwrap())
            .filter(|f| s.iter().any(|&y| f.p_filter.contains(y)))
            .map(|doc| {
                let srch_res = doc.search(&s);
                SearchResult {
                    doc_name: &doc.name,
                    n_hits: srch_res.len(),
                    results: srch_res.into_iter().take(5).collect(),
                }
            })
            .collect();
        Ok(results)
    }
}
