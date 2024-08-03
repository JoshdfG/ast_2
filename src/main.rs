mod lib;
use lib::HTML_ELEMENTS;
fn main() {
    for html in HTML_ELEMENTS {
        let mut result = String::new();
        let mut in_tag = false;

        for char in html.chars() {
            match char {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if in_tag => (),
                _ => result.push(char),
            }
        }

        println!("{}:", html);
        println!("{}\n", result.trim());
    }
}
