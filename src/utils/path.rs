use std::fs;
use std::path::PathBuf;

use crate::types::InputData;

fn get_path(pi: &str) -> Vec<PathBuf> {
    let ps = fs::read_dir(pi).unwrap();
    let mut pv = Vec::<PathBuf>::new();

    ps.filter_map(Result::ok)
        .filter_map(|p| {
            p.path()
                .to_str()
                .and_then(|f| if f.ends_with("json") { Some(p) } else { None })
        })
        .for_each(|p| pv.push(p.path()));
    pv
}

pub fn load_dir(pi: &str) -> Vec<InputData> {
    let path_vec = get_path(pi);
    let mut data_vec: Vec<InputData> = Vec::new();

    for path in path_vec.iter() {
        let f = fs::File::open(path).expect("cannot open");
        let mut d: Vec<InputData> = serde_json::from_reader(f).expect("json");
        data_vec.append(&mut d);
    }

    data_vec
}
