use std::fs::File;
use std::io::BufWriter;
use std::sync::atomic::{AtomicU32, Ordering};

use csv::Reader;
use printpdf::{ColorBits, ColorSpace, Image, ImageTransform, ImageXObject, Mm, PdfDocument, Px};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::entry::{Recipient, RecipientEntry};
use crate::letter::{draw_letter, rgba8_to_rgb8};

pub fn get_recipients(csv_path: String) -> Vec<Recipient> {
  let mut csv = Reader::from_path(csv_path).unwrap();
  let entries: Vec<RecipientEntry> = csv.deserialize().map(|x| x.unwrap()).collect();
  entries
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.clone().into())
    .collect()
}

pub fn generate_pdf<T: Sync, K: Sync, L: Sync>(
  mut recipients: Vec<Recipient>,
  font: Vec<u8>,
  width: f32,
  height: f32,
  output_path: String,
  item_completed_callback: T,
  reset_progress_bar: K,
  complete_progress_bar: L,
) where
  T: Fn(u32) -> (),
  K: Fn() -> (),
  L: Fn() -> (),
{
  let (doc_width, doc_height) = (Mm(width as f64 * 25.4), Mm(height as f64 * 25.4));

  let (doc, page1, layer1) = PdfDocument::new("Letters", doc_width, doc_height, "Text");
  let mut current_page = page1;
  let mut current_layer = layer1;

  let sender = recipients.remove(0);

  let work_items = recipients.len() - 1;

  let completed_items = AtomicU32::new(0);
  let images = recipients
    .into_par_iter()
    .map(|entry| {
      let (letter, width, height) = draw_letter(&font, (width, height), &sender, &entry);
      let letter_rgb = rgba8_to_rgb8(letter.clone(), width, height);

      let image_x = ImageXObject {
        width: Px(width),
        height: Px(height),
        color_space: ColorSpace::Rgb,
        bits_per_component: ColorBits::Bit8,
        interpolate: true,
        image_data: letter_rgb,
        image_filter: None,
        clipping_bbox: None,
      };

      let val = completed_items.fetch_add(1, Ordering::SeqCst);
      item_completed_callback(val);

      Image::from(image_x)
    })
    .collect::<Vec<_>>();

  let mut save_completed_items: u32 = 0;
  reset_progress_bar();
  for image in images {
    let layer_ref = doc.get_page(current_page).get_layer(current_layer);
    image.add_to_layer(layer_ref.clone(), ImageTransform::default());

    if save_completed_items < work_items as u32 {
      let (new_page, new_layer) = doc.add_page(doc_width, doc_height, "Text");
      current_page = new_page;
      current_layer = new_layer;
    }

    save_completed_items += 1;
    item_completed_callback(save_completed_items);
  }
  complete_progress_bar();

  doc
    .save(&mut BufWriter::new(File::create(output_path).unwrap()))
    .unwrap();
}
