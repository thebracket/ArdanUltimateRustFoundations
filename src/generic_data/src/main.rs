use std::ops::Index;

#[derive(Debug)]
struct StableVec<T> {
    data: Vec<Option<T>>
}

impl <T> StableVec<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    fn push(&mut self, item: T) -> usize {
        let id = self.data.len();
        self.data.push(Some(item));
        id
    }

    fn remove(&mut self, id: usize) {
        self.data[id] = None;
    }

    fn get(&self, id: usize) -> &Option<T> {
        &self.data[id]
    }
}

impl<T> Index<usize> for StableVec<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

fn main() {
    let mut store = StableVec::<String>::new();
    let a = store.push("A".to_string());
    let b = store.push("B".to_string());
    let c = store.push("C".to_string());
    store.remove(b);
    println!("{:?}", store.get(a));
    println!("{:?}", store.get(b));
    println!("{:?}", store.get(c));
    println!("{:?}", store[c]);
}
