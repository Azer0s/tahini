use chumsky::Parser;
use tahini::parser::parser;

fn main() {
    let input = "(def a 10)";
    
    let result = parser().parse(input);
    println!("{:?}", result);
}
