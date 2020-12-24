use std::collections::HashMap;

#[derive(Debug)]
pub struct TF<'a> {
    pub terms: HashMap<&'a str, i32>,
    total: i128,
}

impl<'a> TF<'a> {
    pub fn new() -> Self {
        TF {
            terms: HashMap::<&'a str, i32>::new(),
            total: 0i128,
        }
    }

    pub fn insert(&mut self, s: &'a str) {
        let entity = self.terms.entry(s).or_insert(0);
        *entity += 1;
        self.total += 1;
    }

    pub fn get(&self, s: &'a str) -> Option<&i32> {
        match self.terms.get_key_value(s) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    pub fn delete(&mut self, s: &'a str) -> Option<i32> {
        match self.terms.remove(&s) {
            Some(v) => {
                self.total -= i128::from(v);
                Some(v)
            }
            None => None,
        }
    }

    pub fn calculate_tf(self, s: &'a str) -> f32 {
        match self.get(s) {
            Some(v) => (*v as f32)/(self.total as f32),
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
        tf.insert(&"People");
        tf.insert(&"Hello");
        tf.insert(&"People");
        assert_eq!(Some(&2), tf.get(&"People"));
        assert_eq!(Some(&1), tf.get(&"Hello"));
        assert_eq!(None, tf.get(&"World"));
        assert_eq!(3i128, tf.total);
    }

    #[test]
    fn test_delete() {
        let mut tf = TF::new();
        tf.insert(&"People");
        tf.insert(&"Hello");
        tf.insert(&"People");
        assert_eq!(Some(1), tf.delete(&"Hello"));
        assert_eq!(2i128, tf.total);
        assert_eq!(None, tf.delete(&"Hello"));
    }

    #[test]
    fn test_calculate() {
        let mut tf = TF::new();
        tf.insert(&"People");
        tf.insert(&"Hello");
        tf.insert(&"People");
        tf.insert(&"People");
        assert_eq!(0.25f32, tf.calculate_tf(&"Hello"));
    }
}
