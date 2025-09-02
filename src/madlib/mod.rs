use std::collections::HashMap;

use crate::io;

#[derive(Debug)]
struct StoryElement {
    article: String,
    element: String,
}

impl StoryElement {
    fn new(article: String) -> StoryElement {
        StoryElement {
            article: article,
            element: "".to_owned(),
        }
    }
}

fn init_story_elements() -> HashMap<String, StoryElement> {
    HashMap::from([
        (String::from("noun"), StoryElement::new(String::from("a"))),
        (String::from("verb"), StoryElement::new(String::from("a"))),
        (
            String::from("adjective"),
            StoryElement::new(String::from("an")),
        ),
        (
            String::from("adverb"),
            StoryElement::new(String::from("an")),
        ),
    ])
}

pub fn print_story() {
    let mut elements = init_story_elements();

    for element in elements.iter_mut() {
        println!("Enter {} {}: ", element.1.article, element.0);
        let input = io::input::read_input();
        element.1.element = input;
    }
    let noun = elements.get("noun").unwrap();
    let verb = elements.get("verb").unwrap();
    let adjective = elements.get("adjective").unwrap();
    let adverb = elements.get("adverb").unwrap();

    let output = format!(
        "Do you {} your {} {} {}? That's hilarius!",
        verb.element, adjective.element, noun.element, adverb.element
    );
    println!("{}", output);
}
