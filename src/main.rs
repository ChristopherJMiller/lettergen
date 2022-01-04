use std::{io::BufWriter, fs::File};

use andrew::text::load_font_file;
use clap::Parser;
use csv::Reader;
use indicatif::ProgressBar;
use printpdf::{PdfDocument, Mm, ImageXObject, Px, ImageTransform, Image, ColorSpace, ColorBits};

use crate::{entry::{RecipientEntry, Recipient}, cli::Cli, letter::{draw_letter, rgba8_to_rgb8}};

pub mod letter;
pub mod entry;
pub mod cli;

fn main() {
  let args = Cli::parse();
  let font = load_font_file(args.font);
  let mut csv = Reader::from_path(args.csv).unwrap();
  let entries: Vec<RecipientEntry> = csv.deserialize().map(|x| x.unwrap()).collect();
  let valid_entries: Vec<Recipient> = entries.iter().filter(|x| x.is_valid()).map(|x| x.clone().into()).collect();

  let (doc_width, doc_height) = (Mm(args.letter_width as f64 * 25.4), Mm(args.letter_height as f64 * 25.4));

  let (doc, page1, layer1) = PdfDocument::new("Letters", doc_width, doc_height, "Text");
  let mut current_page = page1;
  let mut current_layer = layer1;

  let work_items = valid_entries.len() - 1;

  let progress = ProgressBar::new(work_items as u64);
  for entry in valid_entries.iter().skip(1) {
    let layer_ref = doc.get_page(current_page).get_layer(current_layer);
    let (letter, width, height) = draw_letter(&font, (args.letter_width, args.letter_height), &valid_entries[0], &entry);
    let letter_rgb = rgba8_to_rgb8(letter.clone(), width, height);

    let image_x = ImageXObject {
      width: Px(width),
      height: Px(height),
      color_space: ColorSpace::Rgb,
      bits_per_component: ColorBits::Bit8,
      interpolate: true,
      image_data: letter_rgb,
      image_filter: None,
      clipping_bbox: None
    };

    let image = Image::from(image_x);
    image.add_to_layer(layer_ref.clone(), ImageTransform::default());

    if progress.position() < (work_items - 1) as u64 {
      let (new_page, new_layer) = doc.add_page(doc_width, doc_height, "Text");
      current_page = new_page;
      current_layer = new_layer;
    }

    progress.inc(1);
  }

  doc.save(&mut BufWriter::new(File::create(args.output).unwrap())).unwrap();
}
