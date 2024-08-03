mod lib;
use lib::HTML_ELEMENTS;
fn main() {
    for html in HTML_ELEMENTS {
        let mut result = String::new();
        let mut opening_tag = false;

        for char in html.chars() {
            match char {
                '<' => opening_tag = true,
                '>' => opening_tag = false,
                _ if opening_tag => (),
                _ => result.push(char),
            }
        }

        println!("{}:", html);
        println!("{}\n", result.trim());
    }
}
