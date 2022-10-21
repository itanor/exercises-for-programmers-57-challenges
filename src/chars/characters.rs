use crate::io::input;

pub fn what_is_the_input() { 
  print_text("What is the input string? ");
}

pub fn who_said_it() {
  print_text("Who said it?");
}

pub fn what_is_the_quote() {
  print_text("What is the quote?");
}

fn print_text(to_print: &str) {
  println!("{}", to_print);
}

pub fn count_characters(input: &str) -> usize {
  return input.len();
}

pub fn outputs_the_input(input: &str, total: usize) {
  println!("{} has {} characters", input, total);
}

pub fn outputs(input: &str) {
  println!("{}", input);
}

pub fn read_while_input_has_more_than_zero_chars() {
  let mut must_read_input = true;

  while must_read_input {
    what_is_the_input();
    let input = input::read_input();
    let count = count_characters(&input);
    if count > 0 {
      outputs_the_input(&input, count);
      must_read_input = false;
    }
  }
}
