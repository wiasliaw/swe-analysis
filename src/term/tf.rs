use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TF {
    pub terms: HashMap<String, i32>,
    total: i128,
}

impl TF {
    pub fn new() -> Self {
        TF {
            terms: HashMap::<String, i32>::new(),
            total: 0i128,
        }
    }

    pub fn insert(&mut self, s: String) {
        let entity = self.terms.entry(s).or_insert(0);
        *entity += 1;
        self.total += 1;
    }

    pub fn get(&self, s: String) -> Option<&i32> {
        match self.terms.get_key_value(&s) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    pub fn delete(&mut self, s: String) -> Option<i32> {
        match self.terms.remove(&s) {
            Some(v) => {
                self.total -= i128::from(v);
                Some(v)
            }
            None => None,
        }
    }

    pub fn calculate_tf(self, s: String) -> f32 {
        match self.get(s) {
            Some(v) => (*v as f32) / (self.total as f32),
            None => 0f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tf = TF::new();
        assert!(tf.terms.is_empty());
        assert_eq!(0i128, tf.total);
    }

    #[test]
    fn test_insert() {
        let mut tf = TF::new();
        let cases = vec!["People", "Hello", "People"];
        cases
            .iter()
            .map(|c| c.to_string())
            .for_each(|s| tf.insert(s));
        assert_eq!(Some(&2), tf.get("People".to_string()));
        assert_eq!(Some(&1), tf.get("Hello".to_string()));
        assert_eq!(None, tf.get("World".to_string()));
        assert_eq!(3i128, tf.total);
    }

    #[test]
    fn test_delete() {
        let mut tf = TF::new();
        let cases = vec!["People", "Hello", "People"];
        cases
            .iter()
            .map(|c| c.to_string())
            .for_each(|s| tf.insert(s));
        assert_eq!(Some(1), tf.delete("Hello".to_string()));
        assert_eq!(2i128, tf.total);
        assert_eq!(None, tf.delete("Hello".to_string()));
    }

    #[test]
    fn test_calculate() {
        let mut tf = TF::new();
        let cases = vec!["People", "Hello", "People", "People"];
        cases
            .iter()
            .map(|c| c.to_string())
            .for_each(|s| tf.insert(s));
        assert_eq!(0.25f32, tf.calculate_tf("Hello".to_string()));
    }
}
