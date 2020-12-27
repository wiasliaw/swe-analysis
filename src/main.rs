extern crate rust_stemmers;
extern crate vtext;

use rust_stemmers::{Algorithm, Stemmer};
use vtext::tokenize::{Tokenizer, VTextTokenizerParams};

mod term;
mod types;
mod utils;

use types::StageData;
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

    println!("{:?}", pre_2_data);
}
