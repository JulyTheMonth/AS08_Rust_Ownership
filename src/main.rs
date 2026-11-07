use lc::term::*;
use lc::eval::*;

/// Driver code to run the lambda calculus evaluator.
fn main() {
    // (λx. x y) z
    let input = app(abs("x", app(var("x"), var("y"))), var("z"));

    println!("Original term: {}", input);
    let result = eval(&input);
    println!("Evaluated term: {}", result);
    let test = Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())));

    println!("Test term: {}", test);

// Define Church numerals
// 2 = λf. λx. f (f x)
// 3 = λf. λx. f (f (f x))
let two = abs("f", abs("x", app(var("f"), app(var("f"), var("x")))));
let three = abs("f", abs("x", app(var("f"), app(var("f"), app(var("f"), var("x"))))));
println!("Two terms: {}", two);
    println!("Three terms: {}", three);
// Define addition function for Church numerals

let add = abs("m", abs("n", abs("f", abs("x", app(app(var("m"), var("f")), app(app(var("n"), var("f")), var("x")))))));

// Add two and three
let sum = app(app(add, two), three);



let result = eval(&sum);
println!("The result of adding two and three is: {}", result);  // Should print 5


}