struct Organization {
    pub people: Vec<Person>,    
}

struct Person {
    pub resources: Vec<Resource>,
}

impl Person {
    fn give_resource(&mut self, name: &str, org: &mut Organization, recipient: usize) {
        if let Some((idx, resource)) = self.resources.iter().enumerate().find(|(_, item)| name == item.name) {
            self.resources.remove(idx);
            org.people[recipient].resources.push(resource.clone());
        }
    }
}

#[derive(Clone)]
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
    org.people[0].give_resource("Stapler", &mut org, 1);
}
