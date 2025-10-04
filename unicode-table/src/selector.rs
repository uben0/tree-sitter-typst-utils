use super::Entry;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};

#[derive(Deserialize)]
pub enum Selector {
    #[serde(rename = "charset")]
    Charset(String, Vec<String>),
    #[serde(rename = "union")]
    Union(Vec<Self>),
    #[serde(rename = "intersection")]
    Intersection(Vec<Self>),
    #[serde(rename = "difference")]
    Difference(Vec<Self>),
}

impl Selector {
    pub fn select(&self, charsets: &BTreeMap<&str, Vec<Entry>>) -> HashSet<u32> {
        match self {
            Selector::Charset(charset, labels) => extract(&charsets[charset.as_str()], |label| {
                labels.iter().find(|&l| l == label).is_some()
            }),
            Selector::Union(selectors) => selectors
                .iter()
                .map(|selector| selector.select(charsets))
                .reduce(|lhs, rhs| lhs.union(&rhs).copied().collect())
                .unwrap(),
            Selector::Intersection(selectors) => selectors
                .iter()
                .map(|selector| selector.select(charsets))
                .reduce(|lhs, rhs| lhs.intersection(&rhs).copied().collect())
                .unwrap(),
            Selector::Difference(selectors) => selectors
                .iter()
                .map(|selector| selector.select(charsets))
                .reduce(|lhs, rhs| lhs.difference(&rhs).copied().collect())
                .unwrap(),
        }
    }
}

fn extract(charset: &[Entry], mut filter: impl FnMut(&str) -> bool) -> HashSet<u32> {
    charset
        .iter()
        .filter(|e| filter(e.property))
        .map(|e| e.range.clone())
        .flatten()
        .collect()
}
