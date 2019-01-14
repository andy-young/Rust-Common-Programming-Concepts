// fn main() {
//     use 'mut' override default immutability
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);

// }

// declare constants with all caps, space separated with underscores
// const MAX_POINTS: u32 = 100_000;

// fn main() {
//     // x gets 'shadowed' by the next re-assignment of x
//     let x = 5;
//     // and so on, this x is in the shadow of another re-assignment..
//     let x = x + 1;
//     // this last declaration is now immutable
//     let x = x * 2;

//     println!("The value of x is: {}", x);
// }

// Scalar DataTypes: integer, floating pt number, Bool, char

// Numeric Operations
// ============================================================================

// fn main() {
//     // addition
//     let sum = 5 + 10;
//     println!("The sum of 5 & 10 = {}", sum);

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("The difference of 95.5 and 4.3 = {}", difference);

//     // multiplication
//     let product = 4 * 30;
//     println!("The product of 4 and 30 = {}", product);

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("The quotient of 56.7 and 32.2 is {}", quotient);

//     // remainder
//     let remainder = 43 % 5;
//     println!("The remainder of 43 divided by 5 is {}", remainder);
// }

// The Boolean type
// ============================================================================

// fn main() {
//     let t = true;
//     println!("t = {}", t);

//     let f: bool = false; // with explicit type annotation
//     println!("f = {}", f)
// }

// The Character Type
// fn main() {
//     let c = 'z'; // single chars get single quotes, strings--double quotes
//     println!("c = {}", c);

//     let z = '©';
//     println!("z = {}", z);

//     let heart_eyed_cat = '😻';
//     println!("heart_eyed_cat = {}", heart_eyed_cat);
// }

// Compound DataType: Tuple
// ============================================================================

// fn main() {
//     // let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;
//     println!("The value of x is: {}. y is {}, and z is {}.", x, y, z);
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0; // dot notation and indexing to get tuple value

//     let six_point_four = x.1;

//     let one = x.2;

//     println!(
//         "five_hundred = {}. six_point_four = {}. one = {}.",
//         five_hundred, six_point_four, one
//     );
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let first = a[0];
//     let second = a[1];

//     println!("first = {}. second = {}", first, second);
// }

// Rust Compiler 'Panics' after checking array.length > actual array..
// .. 'Panic' means to exit without an error. Rust protects us by
// immediately exiting instead of allocating memory.

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let index = 10;

//     let el = a[index];

//     println!("el = {}", el);
// }

// Functions
// ============================================================================

// fn main() {
//     println!("I'm about to call 'another_function'!");

//     another_function();
// }

// fn another_function() {
//     println!("'another_function' was called somewhere!");
// }

// Function Parameters
// ============================================================================

// fn main() {
//     another_function(5);
// }

// // In function signatures, we 'must' declare the type of each parameter.
// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// Statements and Expressions in Function Bodies
// ============================================================================

// 'Statements' are an instruction that perform an action..
// .. and do not return a value.

// 'Expressions' to a resulting value;

// let y = 6 --> is a 'statement'.
// Function definitions are also statement.

// fn main() {
//   let y = 6;
// }


// Will error, because can't assign a 'let' statement to variable.
// fn main() {
//   let x = (let y = 6);
// }

// error: expected expression, found statement (`let`)
//    --> src/main.rs:173:12
//     |
// 173 |   let x = (let y = 6);
//     |            ^^^ expected expression
//     |
//     = note: variable declaration using `let` is a statement

// fn main() {
//   let x = 5;

//   let y = {
//       let x = 3;
//       x + 1   // semicolon here will change to statement vs expression
//   };

//   println!("The value of y is: {}", y);
// }

// Functions with Return Values
// ============================================================================

// fn five() -> i32 {
//   5
// }

// fn main() {
//   let x = five();

//   println!("The value of x is: {}", x);
// }

// fn main() {
//   let x = plus_one(5);

//   println!("The value of x is: {}", x);
// }

// fn plus_one(x: i32) -> i32 {
//   x + 1
// }

// fn main() {
//   let x = plus_one(5);

//   println!("The value of x is: {}", x);
// }

// fn plus_one(x: i32) -> i32 {
//   x + 1; // error
// }

// error[E0308]: mismatched types
//    --> src/main.rs:224:28
//     |
// 224 |   fn plus_one(x: i32) -> i32 {
//     |  ____________________________^
// 225 | |   x + 1; // error
//     | |        - help: consider removing this semicolon
// 226 | | }
//     | |_^ expected i32, found ()
//     |
//     = note: expected type `i32`
//                found type `()`

// ============================================================================

