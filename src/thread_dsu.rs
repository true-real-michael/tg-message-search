use std::collections::HashMap;

pub struct ThreadDSU {
    threads: HashMap<usize, Vec<usize>>,
    parents: HashMap<usize, usize>,
}

impl ThreadDSU {
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    pub fn make_set(&mut self, v: usize) {
        self.parents.insert(v, v);
        self.threads.insert(v, vec![v]);
    }

    pub fn find_set(&self, v: usize) -> Option<usize> {
        self.parents.get(&v).copied()
    }

    pub fn union_sets(&mut self, a: usize, b: usize) {
        if let Some(a) = self.find_set(a) {
            if let Some(b) = self.find_set(b) {
                let mut a = a;
                let mut b = b;
                if a != b {
                    if self.threads.get(&a).unwrap().len() < self.threads.get(&b).unwrap().len() {
                        (a, b) = (b, a);
                    }
                    let thread_b = self.threads.remove(&b).unwrap();
                    for v in &thread_b {
                        self.parents.insert(*v, a);
                    }
                    self.threads.get_mut(&a).unwrap().extend(thread_b);
                }
            }
        }
    }

    pub fn get_threads(&self) -> Vec<Vec<usize>> {
        self.threads.values().cloned().collect()
    }

    pub fn get_reverse_mapping(&self) -> HashMap<usize, usize> {
        let mut mapping = HashMap::new();
        for (thread_id, message_ids) in &self.threads {
            for message_id in message_ids {
                mapping.insert(*message_id, *thread_id);
            }
        }
        mapping
    }
}
