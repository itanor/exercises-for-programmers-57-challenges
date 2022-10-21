use crate::{chars, io};

pub fn printing_quotes() {
  chars::characters::what_is_the_quote();
  let quote_phrase = io::input::read_input();

  chars::characters::who_said_it();
  let who_said = io::input::read_input();

  let says = String::from(" says, ");
  let quote = "\"";
  let final_phrase = vec![
      who_said, says, 
      String::from(quote), 
      quote_phrase, 
      String::from(quote)].concat();
  chars::characters::outputs(&final_phrase);
}