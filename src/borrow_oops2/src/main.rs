struct Organization {
    pub people: Vec<Person>,    
}

impl Organization {
    fn move_resource(&mut self, from: usize, to: usize, name: &str) {
        if let Some(resource) = self.people[from].take_resource(name) {
            self.people[to].give_resource(resource);
        }
    }
}

struct Person {
    pub resources: Vec<Resource>,
}

impl Person {
    fn take_resource(&mut self, name: &str) -> Option<Resource> {
        let index = self.resources.iter().position(|r| r.name == name);
        if let Some(index) = index {
            let resource = self.resources.remove(index);
            Some(resource)
        } else {
            None
        }
    }

    fn give_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
}

// #[derive(Clone)] // We don't need this anymore
struct Resource {
    pub name: String,
}

fn main() {
    let mut org = Organization {
        people: vec![
            Person { resources: vec![ Resource { name: "Stapler".to_string() } ]},
            Person { resources: Vec::new() },
        ]
    };
    org.move_resource(0, 1, "stapler");
}
