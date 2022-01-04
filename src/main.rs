use andrew::text::load_font_file;
use clap::Parser;
use csv::Reader;
use image::ColorType;

use crate::{entry::{RecipientEntry, Recipient}, cli::Cli, letter::draw_letter};

pub mod letter;
pub mod entry;
pub mod cli;

fn main() {
  let args = Cli::parse();
  let font = load_font_file(args.font);
  let mut csv = Reader::from_path(args.csv).unwrap();
  let entries: Vec<RecipientEntry> = csv.deserialize().map(|x| x.unwrap()).collect();
  let valid_entries: Vec<Recipient> = entries.iter().filter(|x| x.is_valid()).map(|x| x.clone().into()).collect();
  println!("{:?}", valid_entries);

  let (letter, width, height) = draw_letter(&font, (args.letter_width, args.letter_height), &valid_entries[0], &valid_entries[2]);
  image::save_buffer("test.png", &letter, width as u32, height as u32, ColorType::Rgba8).unwrap();
}
