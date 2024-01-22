// use std::intrinsics::sqrtf64;

fn greet() {
    // Void function
    println!("Hi there!");
}

fn fair_dice_roll() -> i32 {
    // The arrow iindicates the return type
    4
}

fn if_fair_dice_roll(feeling_lucky: bool) -> i32 {
    if feeling_lucky {
        return 6;
    } else {
        return 4;
    }
}

struct Vec2 {
    x: f64,
    y: f64,
}

fn structs() {
    let _v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2: Vec2 = Vec2 { x: 2.0, y: 4.0 };
    let v3: Vec2 = Vec2 { x: 14.0, ..v2 };

    println!("{} {}", v3.x, v3.y);
}

struct Number {
    odd: bool,
    value: i32,
}

fn print_number(n: Number) {
    if let Number { odd: true, value } = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

fn strings() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence: String = String::new();
    sentence.push_str(greeting);
    sentence.push_str(",");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
}

fn collatz_length(mut n: i32) -> i32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

#[rustfmt::skip]
fn pattern_matching(input: char) {

    match input {
        'q' => println!("Quiting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ =>println!("Something else"),
    }

}
fn _main() {
    println!("Hello, world!");

    let x = 13;
    let x = x + 3;

    println!("{}", x);

    let pair: (char, i32) = ('a', 17);

    println!("{} {}", pair.0, pair.1);

    let (some_char, some_int) = ('a', 17);

    println!("Some char {} and some Int {}", some_char, some_int);

    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(4, |x, y| x + y);

    println!("{}", x);

    greet();

    println!("{}", fair_dice_roll());

    // Blocks are expressions which means they evaluate to a value
    // this
    // let x = 42;

    // is equivalent to this:
    let x = { 42 };

    println!("{}", x);

    // inside a block there can be multiple statements
    let x: i32 = {
        let y = 1; // first statement
        let z: i32 = 2; // second statement
        y + z
    };

    println!("{}", x);

    println!("{}", if_fair_dice_roll(true));

    let least: i32 = std::cmp::min(2, 8); // this is 2

    println!("Least {least}");

    structs();

    let one: Number = Number {
        odd: true,
        value: 1,
    };
    let two: Number = Number {
        odd: false,
        value: 2,
    };

    print_number(one);
    print_number(two);

    strings();

    println!("collatz_length {}", collatz_length(11));

    // pattern matching
    pattern_matching('6');
    pattern_matching('w');
}


// Destructuring Tuples
fn describe_point(point: (i32, i32))
{
    match point {
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x < 0 => println!("Left of Y axis"),
        (_, y) if y < 0 => println!("bellow X axis"),
        _ => println!("first quadrant")
    }
}
// Destructuring Arrays

fn des_arrays(){
    let triple: [i32; 3] = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, x] => println!("Firstt element is 0, y = {y}, and z = {x}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut trans: [[i32; 3]; 3] =  [[0; 3]; 3];

    for n in 0..matrix.len() {
        for i in 0..matrix[n].len() {
            trans[i][n] = matrix[n][i]
        }
    }
    trans
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

// Shared references

// A reference provides a way to access another value without taking responisbility for the value, and is
// is called "borrowing". Shared references are read-only, and the referenced data cannot change

fn shared_ref(){
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r {}", *r);
    r = &b;
    println!("r {}", *r)
}

// A shared reference to a type T has type &T. A refenced type is made with the & operator. 
// The * operator "dereferences" a reference, yielding its value

fn magnitude(vector: &[f64; 3]) -> f64{
    let mut mag = 0.0;
    for n in vector{
        mag += n * n;
    }
    
    return mag.sqrt()
}

fn normalize (vector: &mut [f64; 3]){
    let mag = magnitude(&vector);
    vector[0] /= mag;
    vector[1] /= mag;
    vector[2] /= mag;
}

fn main() {
    _main();
    describe_point((-1, 0));
    des_arrays();

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);

    shared_ref();

    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v))
}
