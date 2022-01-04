use andrew::{Endian, Canvas, text::Text};

use crate::entry::Recipient;

const LETTER_ROWS: usize = 20;

fn get_row_y(row: usize, word_height: f32) -> usize {
  (row as f32 * word_height).floor() as usize
}

fn get_centered_x(width: usize, text_width: usize) -> usize {
  (width as f32 / 2.0) as usize - (text_width as f32 / 2.0).round() as usize
}

pub fn draw_letter(font: &Vec<u8>, (letter_width, letter_height): (f32, f32), sender: &Recipient, recipient: &Recipient) -> (Vec<u8>, usize, usize) {
  let (x, y) = ((letter_width * 300.0).floor() as usize, (letter_height * 300.0).floor() as usize);
  let word_height = y as f32 / LETTER_ROWS as f32;
  let mut buf: Vec<u8> = vec![255; 4 * x * y];
  let mut canvas = Canvas::new(&mut buf, x, y, 4 * x, Endian::Little);

  // Return to Sender Block
  const X_MARGIN: usize = 15;
  const Y_MARGIN: usize = 10;
  let sender_line_one = Text::new((X_MARGIN, get_row_y(0, word_height) + Y_MARGIN), [255, 0, 0, 0], font, word_height, 1.0, sender.get_letter_line_one());
  let sender_line_two = Text::new((X_MARGIN, get_row_y(1, word_height) + Y_MARGIN), [255, 0, 0, 0], font, word_height, 1.0, sender.get_letter_line_two());
  let sender_line_three = Text::new((X_MARGIN, get_row_y(2, word_height) + Y_MARGIN), [255, 0, 0, 0], font, word_height, 1.0, sender.get_letter_line_three());
  canvas.draw(&sender_line_one);
  canvas.draw(&sender_line_two);
  canvas.draw(&sender_line_three);

  const START_ROW: usize = 8;
  let mut recipient_line_one = Text::new((0, 0), [255, 0, 0, 0], font, word_height, 1.0, recipient.get_letter_line_one());
  recipient_line_one.pos = (get_centered_x(x, recipient_line_one.get_width()), get_row_y(START_ROW, word_height) + Y_MARGIN);

  let mut recipient_line_two = Text::new((0, 0), [255, 0, 0, 0], font, word_height, 1.0, recipient.get_letter_line_two());
  recipient_line_two.pos = (get_centered_x(x, recipient_line_two.get_width()), get_row_y(START_ROW + 1, word_height) + Y_MARGIN);

  let mut recipient_line_three = Text::new((0, 0), [255, 0, 0, 0], font, word_height, 1.0, recipient.get_letter_line_three());
  recipient_line_three.pos = (get_centered_x(x, recipient_line_three.get_width()), get_row_y(START_ROW + 2, word_height) + Y_MARGIN);

  canvas.draw(&recipient_line_one);
  canvas.draw(&recipient_line_two);
  canvas.draw(&recipient_line_three);

  (buf, x, y)
}

// Workaround for printpdf having a bug with transparency channels (https://github.com/fschutt/printpdf/issues/84)
pub fn rgba8_to_rgb8(input: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
  let mut output = vec![255u8; 3 * width * height];
  for (output, chunk) in output.chunks_exact_mut(3).zip(input.chunks_exact(4)) {
    output.copy_from_slice(&chunk[0..3]);
  }

  return output;
}
