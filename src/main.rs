mod hello;
use hello::what_is_your_name;
use hello::say_hello_to_name;
use hello::read_input;

fn main() {
    what_is_your_name();
    let name = read_input();
    say_hello_to_name(&name);
}
