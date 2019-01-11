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

fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let el = a[index];

    println!("el = {}", el);
}