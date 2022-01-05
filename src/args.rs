use clap::Parser;
use druid::{Data, Lens};

#[derive(Parser, Debug, Clone, Default)]
#[clap(name = "Letter Gen")]
#[clap(author = "Chris M. <hello@chrismiller.xyz>")]
#[clap(version = "1.0")]
#[clap(about = "Formats a CSV of addresses to be printed onto letters.")]
pub struct LetterParams {
  /// File Path to CSV
  pub csv: String,

  /// Width of Physical Letter
  pub letter_width: f32,

  /// Height of Physical Letter
  pub letter_height: f32,

  /// File Path to Font to Use
  pub font: String,

  /// File Path to Output PDF
  pub output: String,
}

#[derive(Data, Clone, Debug, Lens, Default)]
pub struct LetterState {
  pub csv: String,
  pub letter_width: String,
  pub letter_height: String,
  pub output: String,
  pub font: String,
  pub total_work_items: u32,
  pub work_items_complete: u32,
  pub progress: f64,
}

impl LetterState {
  pub fn inputs_valid(&self) -> bool {
    !self.csv.is_empty()
      && !self.letter_width.is_empty()
      && !self.letter_height.is_empty()
      && !self.output.is_empty()
      && self.letter_width.parse::<f32>().is_ok()
      && self.letter_height.parse::<f32>().is_ok()
  }
}
