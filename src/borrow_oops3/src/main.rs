use thiserror::Error;

#[derive(Error, Debug)]
enum OrgError {
    #[error("The requested person does not exist")]
    PersonDoesNotExist(usize),
    #[error("The requested resource is not allocated to the person")]
    ResourceNotFound,
}

struct Organization {
    pub people: Vec<Person>,    
}

impl Organization {
    fn move_resource(&mut self, from: usize, to: usize, name: &str) -> Result<(), OrgError> {
        if let (Some(id1), Some(id2)) = (self.find_person(from), self.find_person(2)) {
            if let Some(resource) = self.people[id1].take_resource(name) {
                self.people[id2].give_resource(resource);
                Ok(())
            } else {
                Err(OrgError::ResourceNotFound)
            }
        } else if self.find_person(from).is_none() {
            return Err(OrgError::PersonDoesNotExist(from));
        } else {
            return Err(OrgError::PersonDoesNotExist(to));
        }
    }

    fn find_person(&self, id: usize) -> Option<usize> {
        self.people.iter().position(|p| p.id == id)
    }
}

struct Person {
    id: usize,
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
            Person { id: 0, resources: vec![ Resource { name: "Stapler".to_string() } ]},
            Person { id: 1, resources: Vec::new() },
        ]
    };
    org.move_resource(0, 1, "stapler").unwrap();
}
