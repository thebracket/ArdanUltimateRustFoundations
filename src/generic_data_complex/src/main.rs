use std::{collections::HashMap, hash::Hash, fmt::Debug};

#[derive(Debug)]
struct HashSetData<KEY, VALUE> 
where KEY: Eq + Hash + std::fmt::Display, VALUE: Debug + Sensor
{
    data: HashMap<KEY, Vec<VALUE>>
}

impl <KEY, VALUE> HashSetData<KEY, VALUE> 
where KEY: Eq + Hash + std::fmt::Display, VALUE: Debug + Sensor
{
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    fn add_reading(&mut self, key: KEY, reading: VALUE) {
        if let Some(entry) = self.data.get_mut(&key) {
            entry.push(reading);
        } else {
            self.data.insert(key, vec![reading]);
        }
    }

    fn print_results(&self) {
        for (key, value) in self.data.iter() {
            let sum: i32 = value.iter().map(|r| r.reading()).sum();
            let avg = sum / value.len() as i32;
            println!("{key} : {avg}");
        }
    }
}

trait Sensor {
    fn reading(&self) -> i32;
}

#[derive(Debug)]
struct Data(i32);

impl Sensor for Data {
    fn reading(&self) -> i32 {
        self.0
    }
}

fn main() {
    let mut readings = HashSetData::<usize, Data>::new();
    readings.add_reading(1, Data(-2));
    readings.add_reading(1, Data(3));
    readings.add_reading(1, Data(5));
    readings.add_reading(2, Data(1));
    readings.print_results();
}
