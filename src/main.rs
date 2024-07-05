fn main() {
    println!("Hello, world!");

}

#[test]
fn hello_test() {
    println!("Hello, world!")
}

#[test]
fn test_variable() {
    let name = "John";
    println!("Hello, {}", name);
}

#[test]
fn test_variable_mutable() {
    let mut name = "John";
    println!("Hello, {}", name);
    name = "Doe";
    println!("Hello, {}", name);
}

#[test]
fn test_static_typing() {
    let name: &str = "John";
    println!("Hello, {}", name);
}

#[test]
fn test_shadowing() {
    let name = "John";
    println!("Hello, {}", name);
    let name = "Doe";
    println!("Hello, {}", name);
}

#[test]
fn test_explicit_data_type() {
    let name: &str = "John";
    let age: i32 = 30;
    let height: f64 = 1.75;
    let is_married: bool = false;

    println!("Hello, {}", name);
    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Married: {}", is_married);
}

#[test]
fn test_number_conversion() {
    let a: i32 = 10;
    let b: i64 = a as i64;
    println!("a: {}", a);
    println!("b: {}", b);
}

#[test]
fn test_numeric_operations() {
    let a = 100;
    let b = 20;
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;
    let quot = a / b;
    let rem = a % b;

    println!("Sum: {}", sum);
    println!("Diff: {}", diff);
    println!("Prod: {}", prod);
    println!("Quot: {}", quot);
    println!("Rem: {}", rem);
}

#[test]
fn test_augmented_assignment() {
    let mut a = 10;
    a += 5;
    println!("a: {}", a);
}

#[test]
fn test_boolean() {
    let is_raining = true;
    let is_sunny = false;

    println!("Is raining: {}", is_raining);
    println!("Is sunny: {}", is_sunny);
}

#[test]
fn test_comparison_operators() {
    let a = 10;
    let b = 20;

    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a <= b: {}", a <= b);
}

#[test]
fn test_logical_operators() {
    let is_raining = true;
    let is_sunny = false;

    println!("Is raining and sunny: {}", is_raining && is_sunny);
    println!("Is raining or sunny: {}", is_raining || is_sunny);
    println!("Is not raining: {}", !is_raining);
}

#[test]
fn test_char() {
    let c = 'A';
    println!("c: {}", c);
}

#[test]
fn test_tuple() {
    let person: (&str, i32, f64, bool) = ("John", 30, 1.75, false);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}", person.2);
    println!("Married: {}", person.3);
}

#[test]
fn test_tuple_destructuring() {
    let person: (&str, i32, f64, bool) = ("John", 30, 1.75, false);
    let (name, age, height, is_married) = person;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Married: {}", is_married);
}

#[test]
fn test_tuple_mutability() {
    let mut person: (&str, i32, f64, bool) = ("John", 30, 1.75, false);
    person.0 = "Doe";
    person.1 = 40;
    person.2 = 1.80;
    person.3 = true;

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}", person.2);
    println!("Married: {}", person.3);
}

#[test]
fn test_tuple_unit() {
    let person: () = ();
    println!("{:?}", person);
}

#[test]
fn test_array() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let number_1 = numbers[0];
    let number_2 = numbers[1];
    let number_3 = numbers[2];
    let number_4 = numbers[3];
    let number_5 = numbers[4];

    // length
    let length: usize = numbers.len();

    println!("Number 1: {}", number_1);
    println!("Number 2: {}", number_2);
    println!("Number 3: {}", number_3);
    println!("Number 4: {}", number_4);
    println!("Number 5: {}", number_5);
    println!("Length: {}", length);

}

#[test]
fn test_array_mutability() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[0] = 10;
    numbers[1] = 20;
    numbers[2] = 30;
    numbers[3] = 40;
    numbers[4] = 50;

    println!("Number 1: {}", numbers[0]);
    println!("Number 2: {}", numbers[1]);
    println!("Number 3: {}", numbers[2]);
    println!("Number 4: {}", numbers[3]);
    println!("Number 5: {}", numbers[4]);
}

#[test]
fn test_array_two_dimensional() {
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    let row_1 = matrix[0];
    let row_2 = matrix[1];
    let row_3 = matrix[2];

    let number_1_1 = matrix[0][0];
    let number_1_2 = matrix[0][1];
    let number_1_3 = matrix[0][2];

    let number_2_1 = matrix[1][0];
    let number_2_2 = matrix[1][1];
    let number_2_3 = matrix[1][2];

    let number_3_1 = matrix[2][0];
    let number_3_2 = matrix[2][1];
    let number_3_3 = matrix[2][2];

    println!("Row 1: {:?}", row_1);
    println!("Row 2: {:?}", row_2);
    println!("Row 3: {:?}", row_3);

    println!("Number 1-1: {}", number_1_1);
    println!("Number 1-2: {}", number_1_2);
    println!("Number 1-3: {}", number_1_3);

    println!("Number 2-1: {}", number_2_1);
    println!("Number 2-2: {}", number_2_2);
    println!("Number 2-3: {}", number_2_3);

    println!("Number 3-1: {}", number_3_1);
    println!("Number 3-2: {}", number_3_2);
    println!("Number 3-3: {}", number_3_3);
}

#[test]
fn test_constants() {
    const PI: f64 = 3.14159;
    println!("PI: {}", PI);
}

#[test]
fn test_variable_scope() {
    let a = 10;
    {
        let b = 20;
        println!("a: {}", a);
        println!("b: {}", b);
    }
    // println!("b: {}", b); // error: cannot find value `b` in this scope
}

#[test]
fn test_string_slice() {
    let mut s: &str = "Hello, world!";
    println!("{}", s);
    s = "Hello, Rust!";
    println!("{}", s);
}

#[test]
fn test_string() {
    let mut name: String = String::from("John");
    name.push_str(" Doe");
    println!("{}", name);

    let hafizh: String = name.replace("John", "Hafizh");
    println!("{}", hafizh);
}




