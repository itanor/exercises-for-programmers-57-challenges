use std::io;
use std::collections::HashMap;

pub fn what_is_your_name() {
  println!("What is your name? ");
}

fn build_greetings() -> HashMap<&'static str, &'static str> {
  let mut greetings = HashMap::new(); 
  greetings.insert("pedro", "Olá, {}, prazer em conhecê-lo!");
  greetings.insert("eric", "Salut {}, ravi de te rencontrer!");
  greetings.insert("john", "Hi, {}, nice to meet you!");
  greetings
}

pub fn say_hello_to_name(name: &str) {
  let greetings = build_greetings();
  let greeting_by_name = match greetings.get(name) {
    Some(n) => n,
    None => "None" 
  };

  let formatted = greeting_by_name.replace("{}", name);
  println!("{}", formatted);
}

pub fn read_input() -> String {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
      Ok(_goes_into_imput) => {},
      Err(_no_uptades) => {},
  }
  input.trim().to_string()
}
