//mod hello;
//use hello::what_is_your_name;
//use hello::say_hello_to_name;
//use hello::read_input;
//mod input;
//use input::read_input;

mod io;
mod chars;
use chars::characters::read_while_input_has_more_than_zero_chars;

fn main() {
    //what_is_your_name();
    //let name = read_input();
    //say_hello_to_name(&name);

    //what_is_the_input();
    //let input = read_input();
    //let count = count_characters(&input);
    //outputs_the_input(&input, count);
    read_while_input_has_more_than_zero_chars();
}
