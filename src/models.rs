use anyhow::Result;
use itertools::Itertools;
use log;
use logging_timer::{stime, time};
use rust_stemmers::{Algorithm, Stemmer};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    convert::From,
    fmt::Display,
    u64,
};
use stop_words;
use unicode_segmentation::UnicodeSegmentation;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Deserialize, PartialEq)]
pub struct RawDoc {
    pub title: String,
    #[serde(rename = "abstract")]
    pub txt_abstract: String,
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Feed {
    pub doc: Vec<RawDoc>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: u64,
    pub title: String,
    pub txt_abstract: String,
    pub url: String,
}

impl From<RawDoc> for Document {
    fn from(base: RawDoc) -> Self {
        Document {
            id: xxh3_64(&base.title.as_bytes()),
            title: base.title,
            txt_abstract: base.txt_abstract,
            url: base.url,
        }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "document id: {}\ndocument name: {}\ntext:{}\n",
            self.id, self.title, self.txt_abstract
        )
    }
}
// #[derive(Debug)]
pub struct Index {
    docs: HashMap<u64, Document>, // Re-add thins once we care about it
    stopwords: HashSet<String>,
    stem_fn: Stemmer,
    pub inverted: HashMap<String, HashSet<u64>>,
}

pub struct SearchResult<'a> {
    pub docs: Vec<&'a Document>,
    // pub meta: SearchMeta
}

impl<'a> From<Vec<&'a Document>> for SearchResult<'a> {
    fn from(src: Vec<&'a Document>) -> Self {
        SearchResult { docs: src }
    }
}

impl<'a> From<Vec<Option<&'a Document>>> for SearchResult<'a> {
    fn from(src: Vec<Option<&'a Document>>) -> Self {
        src.iter()
            .filter(|&f| f.is_some())
            .map(|d| d.unwrap())
            .collect_vec()
            .into()
    }
}

impl Display for SearchResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_strs = self.docs.iter().map(|f| format!("{}", f)).join("\n");
        write!(f, "{}", fmt_strs)
    }
}

impl Index {
    pub fn new() -> Self {
        Index {
            docs: HashMap::new(),
            stopwords: stop_words::get(stop_words::LANGUAGE::English)
                .into_iter()
                .collect(),
            stem_fn: Stemmer::create(Algorithm::English),
            inverted: HashMap::new(),
        }
    }

    #[inline]
    pub fn tokenise(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .unicode_words()
            .filter(|&f| !self.stopwords.contains(f))
            .map(|x| self.stem_fn.stem(x))
            .map(|y| y.to_string())
            .collect()
    }

    pub fn insert<'a>(&'a mut self, doc: Document) -> Result<()> {
        self.docs.insert(doc.id, doc.clone());
        let mut tokens_title = self.tokenise(&doc.title);
        let mut tokens = self.tokenise(&doc.txt_abstract);
        tokens.append(&mut tokens_title);
        for token in tokens.into_iter().unique() {
            let occurrences = self.inverted.entry(token).or_insert({
                let mut a = HashSet::new();
                a.insert(doc.id);
                a
            });
            if !occurrences.contains(&doc.id) {
                occurrences.insert(doc.id);
            };
        }
        Ok(())
    }

    #[time]
    pub fn search(&self, s: &str) -> SearchResult {
        let search_tokens = self.tokenise(s.into());
        // log::info!("Tokens: {:?}", search_tokens);
        let results: Vec<&HashSet<u64>> = search_tokens
            .iter()
            .map(|t| self.inverted.get(t))
            .filter(|f| f.is_some())
            .map(|t| t.unwrap())
            .collect_vec();
        // log::info!("n results found from raw tokens: {}", results.len());
        let isect_ids = self.perform_intersection(results);
        // log::info!("Intersection complete");
        dbg!(&isect_ids);
        isect_ids
            .iter()
            .map(|id| self.docs.get(&id))
            .collect_vec()
            .into()
    }

    #[time]
    fn perform_intersection(&self, values: Vec<&HashSet<u64>>) -> Vec<u64> {
        let res: Vec<u64>;
        if let Some((head, tail)) = values.split_first() {
            let first = head.to_owned().to_owned();
            let mut all_sets = tail.iter();
            dbg!(&first, &all_sets);
            res = first
                .iter()
                .filter(|&k| all_sets.all(|s| s.contains(k)))
                .map(|x| x.to_owned())
                .collect_vec();
        } else {
            res = Vec::new();
        }
        res
    }
}
