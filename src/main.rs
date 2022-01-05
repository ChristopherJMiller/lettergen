use std::env;

use andrew::text::load_font_file;
use clap::Parser;
use druid::{AppLauncher, WindowDesc};
use gen::{generate_pdf, get_recipients};
use indicatif::ProgressBar;
use ui::{build_root_widget, UiDelegate};

use crate::args::{LetterParams, LetterState};

pub mod args;
pub mod entry;
pub mod gen;
pub mod letter;
pub mod ui;

fn main() {
  if env::args().len() > 1 {
    let args = LetterParams::parse();
    let font = load_font_file(args.font);
    let valid_entries = get_recipients(args.csv);
    let progress = ProgressBar::new((valid_entries.len() - 1) as u64);
    generate_pdf(
      valid_entries,
      font,
      args.letter_width,
      args.letter_height,
      args.output,
      |_| progress.inc(1),
    );
  } else {
    // Launch UI
    let main_window = WindowDesc::new(build_root_widget).title("Letter Gen").resizable(false);

    // create the initial app state
    let initial_state = LetterState {
      font: "font.ttf".to_string(),
      ..Default::default()
    };

    // start the application
    AppLauncher::with_window(main_window)
      .delegate(UiDelegate)
      .launch(initial_state)
      .expect("Failed to launch application");
  }
}
