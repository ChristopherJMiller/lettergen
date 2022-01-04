use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "Letter Gen")]
#[clap(author = "Chris M. <hello@chrismiller.xyz>")]
#[clap(version = "1.0")]
#[clap(about = "Formats a CSV of addresses to be printed onto letters.")]
pub struct Cli {
  /// File Path to CSV
  pub csv: String,

  /// Width of Physical Letter
  pub letter_width: f32,

  /// Height of Physical Letter
  pub letter_height: f32,

  /// File Path to Font to Use
  pub font: String
}
