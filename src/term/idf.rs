use std::collections::HashMap;

#[derive(Debug)]
pub struct IDF<'a> {
    pub total_docs: i128,
    pub terms: HashMap<&'a str, i32>,
}

impl<'a> IDF<'a> {
    pub fn new() -> Self {
        Self {
            total_docs: 0,
            terms: HashMap::<&'a str, i32>::new(),
        }
    }

    pub fn insert(&mut self, s: &'a str) {
        let entity = self.terms.entry(s).or_insert(0);
        *entity += 1;
    }

    pub fn get(&self, s: &'a str) -> Option<&i32> {
        match self.terms.get_key_value(s) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    pub fn delete(&mut self, s: &'a str) -> Option<i32> {
        self.terms.remove(&s)
    }

    pub fn update_total(&mut self, t: i128) {
        self.total_docs = t;
    }

    pub fn calculate_idf(&self, s: &'a str) -> f32 {
        match self.get(s) {
            Some(v) => {
                let tmp = (self.total_docs as f32).abs() / (*v as f32);
                tmp.log10()
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
        idf.insert(&"People");
        idf.insert(&"Hello");
        idf.insert(&"People");
        assert_eq!(Some(&2), idf.get(&"People"));
        assert_eq!(Some(&1), idf.get(&"Hello"));
        assert_eq!(None, idf.get(&"World"));
    }

    #[test]
    fn test_delete() {
        let mut idf = IDF::new();
        idf.insert(&"People");
        idf.insert(&"Hello");
        idf.insert(&"People");
        assert_eq!(Some(1), idf.delete(&"Hello"));
        assert_eq!(None, idf.delete(&"Hello"));
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
            idf.insert(&"People");
        }
        for _ in 0..100 {
            idf.insert(&"Hello");
        }
        idf.update_total(100);
        assert_eq!(1f32, idf.calculate_idf(&"People"));
        assert_eq!(0f32, idf.calculate_idf(&"Hello"));
    }
}
