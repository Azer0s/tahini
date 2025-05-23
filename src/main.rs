use chumsky::Parser;
use tahini::parser::var_type;
use tahini::parser::parser;
use tahini::parser::statement;

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
    
    // Test with a super simple if statement
    let simple_if = "(if true 1 2)";
    let result = statement().parse(simple_if);
    println!("Simple if statement result: {:?}", result);
    
    // Test with a simple call
    let call_test = "(print 123)";
    let result = statement().parse(call_test);
    println!("Simple call result: {:?}", result);
    
    // Test with a more complex call
    let call_test2 = "(< a b)";
    let result = statement().parse(call_test2);
    println!("Call with operator result: {:?}", result);
    
    // Test with a more complex if statement
    let test2 = "(if (< a b) a b)";
    let result = statement().parse(test2);
    println!("Complex if statement result: {:?}", result);
    
    // Test fibonacci example
    let test3 = "(if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))";
    let result = statement().parse(test3);
    println!("Fibonacci if statement result: {:?}", result);
    
    // Test the whole fibonacci function definition
    let full_str = "(def fib (fn [(:n i32)] i32 (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))";
    let full_result = parser().parse(full_str);
    println!("Full fibonacci function result: {:?}", full_result);
}
