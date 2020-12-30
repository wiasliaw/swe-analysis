use std::io::Write;
use std::fs::File;

extern crate rust_stemmers;
extern crate vtext;

use rust_stemmers::{Algorithm, Stemmer};
use vtext::tokenize::{Tokenizer, VTextTokenizerParams};

mod term;
mod types;
mod utils;

use term::idf::IDF;
use term::tf::TF;
use types::{LabelData, StageData};
use utils::common::{contains_common, contains_illegal};

fn main() {
    // init
    let tok = VTextTokenizerParams::default().lang("en").build().unwrap();
    let en_stemmer = Stemmer::create(Algorithm::English);

    // load
    let data = utils::path::load_dir("./sample");

    // tokenize
    let pre_1_data: Vec<StageData> = data
        .iter()
        .map(|d| StageData {
            title: d.title.to_owned(),
            term_split: tok
                .tokenize(&d.description[..])
                .map(|t| t.to_string())
                .collect(),
        })
        .collect();

    // stemming
    let pre_2_data: Vec<StageData> = pre_1_data
        .iter()
        .map(|s| StageData {
            title: s.title.to_owned(),
            term_split: s
                .term_split
                .iter()
                .filter_map(|t| {
                    if !contains_common(t.clone()) && !contains_illegal(t.clone()) {
                        Some(t)
                    } else {
                        None
                    }
                })
                .map(|t| en_stemmer.stem(t).to_string())
                .collect(),
        })
        .collect();

    // idf
    let mut idf = IDF::new();
    for data in pre_2_data.iter() {
        idf.insert(data.term_split.clone());
    }

    // tf
    let mut pre_3_data: Vec<Vec<LabelData>> = Vec::new();

    for data in pre_2_data.iter() {
        let mut tf = TF::new();
        data.term_split
            .iter()
            .for_each(|t| tf.insert(t.to_string()));
        let d: Vec<LabelData> = data
            .term_split
            .iter()
            .map(|t| {
                let tf = tf.clone();
                LabelData {
                    term: t.to_string(),
                    tf: tf.calculate_tf(t.to_string()),
                    idf: idf.clone().calculate_idf(t.to_string()),
                }
            })
            .collect();
        pre_3_data.push(d);
    }

    // write
    let mut f = File::create("./tmp/pre_3_data.json").unwrap();
    write!(&mut f, "{:#?}", pre_3_data);
}
