use std::fmt::Display;
use std::collections::HashMap;

struct Cat {
    name: String,
    status: String,
}

impl Display for Cat {
    /// Print service for cats
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.status)
    }
}

struct CatOwner {
    cat_id: usize,
}

/// Provides a central storage location for cats
struct CatStore {
    next_cat: usize,
    cats: HashMap<usize, Cat>,
}

impl CatStore {
    /// Creates a new CatStore
    fn new() -> Self {
        Self {
            next_cat: 0,
            cats: HashMap::new(),
        }
    }

    /// Add a new cat, and return its ID number
    fn add_cat(&mut self, cat: Cat) -> usize {
        let id = self.next_cat;
        self.next_cat += 1;

        self.cats.insert(id, cat);
        id
    }

    /// Find cat by id, set status to "purring"
    fn feed_cat(&mut self, id: usize) {
        if let Some(mut cat) = self.cats.get_mut(&id) {
            cat.status = "Purring".to_string();
        }
    }
}

pub fn feed_cats_by_id(n_cats: usize) {
    let mut store = CatStore::new();
    let mut owners = Vec::new();
    for i in 0 .. n_cats {
        // Make a cat
        let new_cat = Cat{ 
            name: format!("Fuzzy Friend {}", i+1),
            status: String::new(),
        };
        // Add it to the central cat store and get ID
        let new_id = store.add_cat(new_cat);

        // Associate the owner with the ID
        owners.push(
            CatOwner { cat_id: new_id }
        );
    }

    // Start the timer
    let now = std::time::Instant::now();
    owners
        .iter()
        .for_each(|owner| store.feed_cat(owner.cat_id));
    let duration = now.elapsed();

    super::print_result("Cat Store", duration);
}