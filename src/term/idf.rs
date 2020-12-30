use std::collections::HashMap;

#[derive(Debug)]
pub struct IDF {
    pub total_docs: i128,
    pub terms: HashMap<String, i32>,
}

impl IDF {
    pub fn new() -> Self {
        Self {
            total_docs: 0,
            terms: HashMap::<String, i32>::new(),
        }
    }

    pub fn insert(&mut self, s: String) {
        let entity = self.terms.entry(s).or_insert(0);
        *entity += 1;
    }

    pub fn get(&self, s: String) -> Option<&i32> {
        match self.terms.get_key_value(&s) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    pub fn delete(&mut self, s: String) -> Option<i32> {
        self.terms.remove(&s)
    }

    pub fn update_total(&mut self, t: i128) {
        self.total_docs = t;
    }

    pub fn calculate_idf(&self, s: String) -> f32 {
        match self.get(s) {
            Some(v) => {
                let tmp = ((self.total_docs as f32) / (*v as f32)).log10();
                tmp
            }
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
        let cases = vec!["People", "Hello", "People"];
        cases
            .iter()
            .map(|c| c.to_string())
            .for_each(|s| {
                idf.insert(s)
            });

        assert_eq!(Some(&2), idf.get("People".to_string()));
        assert_eq!(Some(&1), idf.get("Hello".to_string()));
        assert_eq!(None, idf.get("World".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut idf = IDF::new();
        let cases = vec!["People", "Hello", "People"];
        cases
            .iter()
            .map(|c| c.to_string())
            .for_each(|s| {
                idf.insert(s)
            });

        assert_eq!(Some(1), idf.delete("Hello".to_string()));
        assert_eq!(None, idf.delete("Hello".to_string()));
    }

    #[test]
    fn test_update_total() {
        let mut idf = IDF::new();
        idf.update_total(42);
        assert_eq!(42i128, idf.total_docs);
    }

    #[test]
    fn test_calculate() {
        let mut idf = IDF::new();
        for _ in 0..10 {
            idf.insert("People".to_string());
        }
        for _ in 0..100 {
            idf.insert("Hello".to_string());
        }
        idf.update_total(100);
        assert_eq!(2f32, idf.calculate_idf("People".to_string()));
        assert_eq!(0f32, idf.calculate_idf("Hello".to_string()));
    }
}
