use csv::Reader;

use crate::entry::{RecipientEntry, Recipient};

pub mod entry;

fn main() {
  let mut csv = Reader::from_path("test.csv").unwrap();
  let entries: Vec<RecipientEntry> = csv.deserialize().map(|x| x.unwrap()).collect();
  let valid_entries: Vec<Recipient> = entries.iter().filter(|x| x.is_valid()).map(|x| x.clone().into()).collect();
  println!("{:?}", valid_entries);
}
