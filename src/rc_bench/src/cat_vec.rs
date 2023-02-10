use std::fmt::Display;

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
    cat_idx: usize,
}

/// Provides a central storage location for cats
struct CatStore {
    cats: Vec<Cat>,
}

impl CatStore {
    /// Creates a new CatStore
    fn new() -> Self {
        Self {
            cats: Vec::new(),
        }
    }

    /// Add a new cat, and return its ID number
    fn add_cat(&mut self, cat: Cat) -> usize {
        let id = self.cats.len();
        self.cats.push(cat);
        id
    }

    /// Find cat by id, set status to "purring"
    fn feed_cat(&mut self, id: usize) {
        self.cats[id].status = "Purring".to_string();
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
            CatOwner { cat_idx: new_id }
        );
    }

    // Start the timer
    let now = std::time::Instant::now();
    owners
        .iter()
        .for_each(|owner| store.feed_cat(owner.cat_idx));
    let duration = now.elapsed();

    super::print_result("Vector of Cats", duration);
}