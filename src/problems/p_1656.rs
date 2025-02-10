use std::collections::HashMap;

#[derive(Default)]
struct OrderedStream {
    store: HashMap<usize, String>,
    ptr: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        Default::default()
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id_key = (id_key - 1) as usize;
        self.store.insert(id_key, value);

        let mut res = Vec::new();
        if id_key == self.ptr {
            while let Some(val) = self.store.get(&self.ptr) {
                res.push(val.clone());
                self.ptr += 1;
            }
        }
        res
    }
}
