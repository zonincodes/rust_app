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

fn print_number(n: Number){
    if let Number{odd: true, value } =n {
        println!("Odd number: {}", value);
    } else if let Number {odd: false, value} = n {
        println!("Even number: {}", value);
    }
}


fn strings(){
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence: String = String::new();
    sentence.push_str(greeting);
    sentence.push_str(",");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
}


fn collatz_length(mut n: i32) -> i32{
    let mut len = 1;
    while n > 1{
        n = if n % 2 == 0 { n / 2} else { 3* n + 1};
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length(){
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
fn main() {
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

    let one: Number = Number {odd: true, value: 1};
    let two: Number = Number {odd: false, value: 2};

    print_number(one);
    print_number(two);

    strings();

    println!("collatz_length {}", collatz_length(11));

    // pattern matching
    pattern_matching('5')

}
