use std::io;

pub fn read_input() -> String {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
      Ok(_goes_into_imput) => {},
      Err(_no_uptades) => {},
  }
  input.trim().to_string()
}
