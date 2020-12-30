use std::collections::HashMap;

#[derive(Debug)]
pub struct IDF {
    pub total_docs: i128,
    pub terms: HashMap<String, i32>,
}

impl Clone for IDF {
    fn clone(&self) -> Self {
        Self {
            total_docs: self.total_docs.clone(),
            terms: self.terms.clone(),
        }
    }
}

impl IDF {
    pub fn new() -> Self {
        Self {
            total_docs: 0,
            terms: HashMap::<String, i32>::new(),
        }
    }

    pub fn insert(&mut self, s: Vec<String>) {
        self.total_docs += 1;
        let mut tmp: HashMap<String, bool> = HashMap::new();
        
        s.iter().for_each(|t| {
            let tmp_entity = tmp.entry(t.to_string()).or_insert(false);
            if !*tmp_entity {
                let term_entity = self.terms.entry(t.to_string()).or_insert(0);
                *term_entity += 1;
            }
            *tmp_entity = true;
        });
    }

    pub fn get(self, s: String) -> Option<i32> {
        match self.terms.get(&s) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn calculate_idf(self, s: String) -> f32 {
        let total = self.total_docs.clone();
        match self.get(s) {
            Some(v) => ((total as f32) / (v as f32)).log10(),
            None => 0f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let idf = IDF::new();
        assert!(idf.terms.is_empty());
        assert_eq!(0i128, idf.total_docs);
    }

    #[test]
    fn test_insert() {
        let mut idf = IDF::new();
        let cases: Vec<Vec<&str>> = vec![
            vec!["Hello", "World", "Hello"],
            vec!["Hello", "Ella"],
        ];
        cases.iter().for_each(|vs| {
            let s: Vec<String> = vs.iter().map(|s| s.to_string()).collect();
            idf.insert(s);
        });
        assert_eq!(Some(2), idf.clone().get("Hello".to_string()));
        assert_eq!(Some(1), idf.clone().get("World".to_string()));
        assert_eq!(2i128, idf.clone().total_docs);
    }

    #[test]
    fn test_calculate_idf() {
        let mut idf = IDF::new();
        let cases: Vec<Vec<&str>> = vec![
            vec!["Hello", "World", "Hello"],
            vec!["Hello", "Ella"],
        ];
        cases.iter().for_each(|vs| {
            let s: Vec<String> = vs.iter().map(|s| s.to_string()).collect();
            idf.insert(s);
        });
        assert_eq!(0f32, idf.clone().calculate_idf("Hello".to_string()));
        assert_eq!(2f32.log10(), idf.clone().calculate_idf("World".to_string()));
    }
}
