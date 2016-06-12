use ansi_term::Colour::*;

pub fn green(string: &str) -> String {
  String::from(&*Green.bold().paint(string))
}

pub fn red(string: &str) -> String {
  String::from(&*Red.bold().paint(string))
}
