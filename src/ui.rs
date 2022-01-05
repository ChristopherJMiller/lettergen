use std::path::{Path, PathBuf};
use std::thread;

use andrew::text::load_font_file;
use druid::widget::{Align, Button, Flex, Label, ProgressBar, TextBox};
use druid::{
  commands, AppDelegate, Command, DelegateCtx, Env, ExtEventSink, FileDialogOptions, FileSpec, Handled, Selector,
  Target, Widget, WidgetExt,
};

use crate::args::LetterState;
use crate::gen::{generate_pdf, get_recipients};

const PROGRESS_TOTAL_ITEMS: Selector<u32> = Selector::new("progress_total_items");
const PROGRESS: Selector<u32> = Selector::new("generate_pdf_progress");

pub fn handle_generate_pdf(sink: ExtEventSink, data: LetterState) {
  thread::spawn(move || {
    let font = load_font_file(data.font);
    let valid_entries = get_recipients(data.csv);
    let output_file = if Path::new(&data.output).extension().is_some() {
      data.output.clone()
    } else {
      let mut path = PathBuf::from(data.output.as_str());
      path.set_extension("pdf");
      path.to_string_lossy().to_string()
    };
    sink
      .submit_command(PROGRESS_TOTAL_ITEMS, valid_entries.len() as u32 - 1, Target::Auto)
      .expect("Failed to send command");
    generate_pdf(
      valid_entries,
      font,
      data.letter_width.parse::<f32>().unwrap(),
      data.letter_height.parse::<f32>().unwrap(),
      output_file,
      |total_complete| {
        sink
          .submit_command(PROGRESS, total_complete, Target::Auto)
          .expect("Failed to send command");
      },
    );
    sink
      .submit_command(PROGRESS_TOTAL_ITEMS, 0, Target::Auto)
      .expect("Failed to send command");
  });
}

pub fn build_root_widget() -> impl Widget<LetterState> {
  let csv = FileSpec::new("Comma Separated Values File", &["csv"]);

  let title = Label::new("Letter Gen").with_text_size(36.0).align_left();

  let csv_label = Label::new("Address Book File:").align_left();

  let csv_file = FileDialogOptions::new()
    .allowed_types(vec![csv])
    .default_type(csv)
    .name_label("CSV")
    .title("Select your input file of recipients")
    .button_text("Choose File");

  let csv_file_button = Button::new("Choose File")
    .on_click(move |ctx, _, _| ctx.submit_command(commands::SHOW_OPEN_PANEL.with(csv_file.clone())));

  let csv_path = Label::new(|data: &LetterState, _: &Env| data.csv.clone());

  let csv_row = Flex::row()
    .with_child(csv_file_button)
    .with_spacer(10.0)
    .with_child(csv_path)
    .align_left();

  let letter_dim_label = Label::new("Letter Size: ");
  let letter_width_input = TextBox::new()
    .with_placeholder("Width")
    .with_text_size(18.0)
    .lens(LetterState::letter_width);
  let letter_height_input = TextBox::new()
    .with_placeholder("Height")
    .with_text_size(18.0)
    .lens(LetterState::letter_height);
  let letter_in_label = Label::new(" in.");
  let letter_x_label = Label::new(" in. x ");

  let letter_size_row = Flex::row()
    .with_child(letter_dim_label)
    .with_child(letter_width_input)
    .with_child(letter_x_label)
    .with_child(letter_height_input)
    .with_child(letter_in_label)
    .align_left();

  let output_file_label = Label::new("Output File Name: ");
  let output_file_input = TextBox::new().with_text_size(18.0).lens(LetterState::output);
  let output_file_row = Flex::row()
    .with_child(output_file_label)
    .with_spacer(5.0)
    .with_child(output_file_input)
    .align_left();

  let gen_button = Button::new("Generate Letters")
    .expand_width()
    .on_click(move |ctx, data: &mut LetterState, _env| {
      if data.inputs_valid() {
        handle_generate_pdf(ctx.get_external_handle(), data.clone());
      }
    });

  let progress = ProgressBar::new().expand_width().lens(LetterState::progress);

  let layout = Flex::column()
    .with_child(title)
    .with_spacer(20.0)
    .with_child(csv_label)
    .with_spacer(5.0)
    .with_child(csv_row)
    .with_spacer(10.0)
    .with_child(letter_size_row)
    .with_spacer(10.0)
    .with_child(output_file_row)
    .with_spacer(10.0)
    .with_child(gen_button)
    .with_spacer(5.0)
    .with_child(progress)
    .padding(10.0);

  Align::left(layout)
}

pub struct UiDelegate;

impl AppDelegate<LetterState> for UiDelegate {
  fn command(
    &mut self,
    _ctx: &mut DelegateCtx,
    _target: Target,
    cmd: &Command,
    data: &mut LetterState,
    _env: &Env,
  ) -> Handled {
    if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
      data.csv = file_info.path().to_str().unwrap().to_string();
      return Handled::Yes;
    }
    if let Some(&total) = cmd.get(PROGRESS_TOTAL_ITEMS) {
      data.work_items_complete = 0;
      data.total_work_items = total;
      data.progress = 0.0;
    }
    if let Some(&progress) = cmd.get(PROGRESS) {
      data.work_items_complete = progress;
      if data.total_work_items > 0 {
        data.progress = data.work_items_complete as f64 / data.total_work_items as f64;
      }
    }
    Handled::No
  }
}
