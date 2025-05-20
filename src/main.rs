use chumsky::Parser;
use tahini::parser::var_type;
use tahini::parser::parser;

fn main() {
    // Test the generic struct parser directly
    let input = "(struct<T> (:a T) (:b f64))";
    let result = var_type().parse(input);
    println!("Generic struct parse result: {:?}", result);
    
    let input = "(struct<K V> (:a K) (:b V))";
    let result = var_type().parse(input);
    println!("Generic struct with multiple params: {:?}", result);
    
    // Original code
    let input = "(def a 10)";
    let result = parser().parse(input);
    println!("Original parser result: {:?}", result);
}
