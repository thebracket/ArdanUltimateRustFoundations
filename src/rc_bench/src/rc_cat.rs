use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct Cat {
    name: String,
    status: RefCell<String>,
}

impl Display for Cat {
    /// Print service for cats
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.status.borrow())
    }
}

struct CatOwner {
    cat: Rc<Cat>,
}

impl CatOwner {
    fn feed_cat(&self) {
        let mut borrow = self.cat.status.borrow_mut();
        *borrow = "Purring".to_string();
    }
}

pub fn feed_cats(n_cats: usize) {
    let mut owners = Vec::new();
    for i in 0 .. n_cats {
        // Make a cat
        let new_cat = Rc::new(Cat{ 
            name: format!("Fuzzy Friend {}", i+1),
            status: RefCell::new(String::new()),
        });
        
        // Associate the owner with the ID
        owners.push(
            CatOwner { cat: new_cat.clone() }
        );
    }

    // Start the timer
    let now = std::time::Instant::now();
    owners
        .iter()
        .for_each(|owner| owner.feed_cat());
    let duration = now.elapsed();

    super::print_result("RC Cats", duration);
}