mod first;
mod second;
mod model;
mod third;

use core::str;
use std::{cell::RefCell, collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque}, fmt::format, ops::Deref, rc::Rc, result, vec};

use first::say_hello;
use second::say_hello as second_say_hello;

#[test]
fn test_use() {
    say_hello();
    second_say_hello();
    first::second::third::say_hello();
}

use model::User;

#[test]
fn test_module() {
    let user = User {
        first_name: String::from("Irfan"),
        last_name: String::from("Bakhtiar"),
        username: String::from("irfanbakhtiar"),
        email: String::from("irfanbakhtiar.dev@gmail.com"),
        age: 20
    };

    user.say_hello("irfan");
}

fn main() {
    println!("Hello, world!");
    println!("Hello, Irfan!");
    println!("Hello, Budi!");
}

#[test]
fn hello_test() {
    println!("Hello Test")
}

#[test]
fn test_variable() {
    let name = "Muhammad Irfan Bakhtiar";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Muhammad Irfan Bakhtiar";
    println!("Hello {}", name);
    
    name = "Budi Nugraha";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Muhammad Irfan Bakhtiar";
    println!("Hello {}", name);
    
    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Muhammad Irfan Bakhtiar";
    println!("Hello {}", name);
    
    let name = 10;
    println!("Hello {}", name);
}

/*
    ini komentar lebih dari satu baris
*/
#[test]
fn comment() {
    // ini komentar

    print!("Hello");
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a = 10;
    print!("{}", a);

    let b = 10.5;
    print!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    print!("{}", a);

    let b: i16 = a as i16;
    print!("{}", b);

    let c: i32 = a as i32;
    print!("{}", c);

    let d: i64 = 10000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    
    let c = a * b;
    println!("{}", c);

    let d = a / b;
    println!("{}", d);
    
    let e = a + b;
    println!("{}", e);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;

    a += 10;
    println!("{}", a);
    
    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;

    let result: bool = a > b;
    println!("{}", result);
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;

    println!("{}", lulus);
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c: bool = data.2;

    let (a, b, c) = data;
    println!("{} {} {}", a , b, c);
    
    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let mut array: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [
        [1,2],
        [3,4]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope() {
    println!("{}", MAXIMUM);

    let irfan = 1;

    {
        println!("{}", irfan);

        let bakhtiar = 2;

        println!("{}", bakhtiar);
    }

    // println!("{}", bakhtiar);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Irfan");

    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Bakhtiar");

    println!("{}, {}", a, b);
}

#[test]
fn string() {
    let name: &str = "   Muhammad Irfan Bakhtiar   ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut  name: String = String::from("Muhammad Irfan");
    name.push_str(" Bakhtiar");
    println!("{}", name);

    let budi = name.replace("Irfan", "Budi");
    println!("{}", budi);
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    println!("{}", a);
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("irfan");
    println!("{}", name1);

    let name2: String = name1;
    println!("{}", name2);
    // println!("{}", name1);
}

#[test]
fn clone() {
    let name1 = String::from("Irfan");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 7;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6{
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }

        number += 1;
    }
}

#[test]
fn while_loop () {
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }
        
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    
    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range() {
    let range = 0..5;
    println!("Start {}", range.start);
    println!("End {}", range.end);

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;
    println!("Start {}", range.start());
    println!("End {}", range.end());

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

// fn say_hello() {
//     println!("Hello");
// }

#[test]
fn test_say_hello() {
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Irfan", "Muhammad");
    say_goodbye("Budi", "Doremi");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result: i32 = factorial_loop(5);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Irfan"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return  1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(number: i32) {
    println!("number {}", number);
}

fn hi(name: String) {
    println!("name {}", name)
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Irfan");
    hi(name);
    // println!("{}", name);
} 

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Irfan");
    let last_name = String::from("Bakhtiar");
    let full_name = full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String) {
    value.push_str("TEST");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Irfan");
    let value_borrow = &mut value;

    change_value(value_borrow);
    change_value(value_borrow);
    change_value(value_borrow);
    
    println!("{}", value);
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Irfan");
    let last_name = String::from("Bakhtiar");
    let full_name = get_full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);
    
    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name = String::from("Muhamma Irfan Bakhtiar");

    let first_name: &str = &name[0..3];
    println!("{}", first_name);

    let last_name: &str = &name[14..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {} my name is {}", name, self.first_name)
    }
}

fn print_person(person: &Person){
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Muhammad");
    let last_name = String::from("Irfan");

    let person: Person = Person {
        first_name,
        middle_name: String::from("Irfan"),
        last_name,
        age: 25
    };

    print_person(&person);

    let person2 = Person{
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person2);

    println!("{}", person.first_name);
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point: GeoPoint = GeoPoint::new(10.0, 10.0);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.23232, 100.91);
    println!("long : {}", geo_point.0);
    println!("long : {}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing{};
}

#[test]
fn test_method() {
    let person = Person{
        first_name: String::from("Irfan"),
        middle_name: String::from("Irfan"),
        last_name: String::from("Irfan"),
        age: 25
    };

    person.say_hello("Budi");
}

enum Level {
    Regular, 
    Premium,
    Platinum
}

#[test]
fn test_enum() {
    let level: Level = Level::Premium;

    match level {
        Level::Regular => {
            println!("Regular")
        }

        Level::Premium => {
            print!("Premium")
        }

        Level::Platinum => {
            println!("Platinum")
        }
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with E-Wallet {} {} amount {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("316316813381"));
    _payment1.pay(1212);

    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("112612612"));
    _payment2.pay(331131);
    
    let _payment3: Payment = Payment::EWallet(String::from("Gopay"), String::from("112612612"));
    _payment3.pay(1312312);
}

#[test]
fn test_match_value() {
    let name = "Irfan";

    match name {
        "Irfan" => {
            println!("Hello Irfan");
        }
        "Budi" => {
            println!("Hello Budi");
        }
        other => {
            println!("Hello {}", other)
        }
    }

    match name {
        "Irfan" | "Budi" => {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other)
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 100;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Not Bad")
        }
        other => {
            println!("Invalid value {}", other)
        }
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint::new(0.0, 1.1);

    match point {
        GeoPoint(long, 0.0 ) => {
            println!("long : {}", long);
        }
        GeoPoint(0.0, lat ) => {
            println!("lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long : {}, lat : {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("irfan"),
        middle_name: String::from("muhammad"),
        last_name: String::from("bakhtiar"),
        age: 25,
    };

    match person {
        Person { first_name, last_name, .. } => {
            println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(0.0, 1.1);

    match point {
        GeoPoint(long, _ ) => {
            println!("long : {}", long);
        }
    }
}

#[test]
fn test_ignoring_range() {
    let value = 100;
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Not Bad")
        }
        _ => {
            println!("Invalid value")
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 9;
    let result = match value {
        0 => "nol",
        1 => {
            "satu"
        },
        2 => {
            "dua"
        },
        _ => "invalid"
    };

    println!("{}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age
}

type Pelanggan = Customer;

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("12311231"),
        name: String::from("Irfan"),
        age: 25
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye {
    fn goodbye(&self) -> String;
    fn goodbye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person {
    fn goodbye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }
    fn goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello());
}

fn hello_and_goodybe(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.say_hello());
    println!("{}", value.goodbye());
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Muhammad"),
        middle_name: String::from("Irfan"),
        last_name: String::from("Bakhtiar"),
        age: 25
    };

    say_hello_trait(&person);
    hello_and_goodybe(&person);

    let result = person.say_hello_to("Budi");
    println!("{}", result);

    let result = person.hello();
    println!("{}", result);

    println!("{}", person.goodbye());
    println!("{}", person.goodbye_to("Irfan"));

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Budi");
}

struct SimplePerson {
    name: String
}

impl CanSayGoodBye for SimplePerson {
    fn goodbye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson { name }
} 

#[test]
fn test_return_trait() {
    let person= create_person(String::from("Irfan"));
    println!("{}", person.goodbye());
    println!("{}", person.goodbye_to("Budi"));
}

trait CanSay: CanSayHello + CanSayGoodBye {
    
}

struct  SimpleMan {
    name: String
}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}

impl CanSayGoodBye for SimpleMan {
    fn goodbye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

impl CanSay for SimpleMan {

}

struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_struct() {
    let integer: Point<i32> = Point::<i32> {
        x:1, y:2
    };
    let float: Point<f64> = Point::<f64> {
        x:1.0, y:2.1
    };

    println!("{} {}", integer.x, integer.y);
    println!("{} {}", float.x, float.y);
}

enum Value<T> {
    NONE,
    VALUE(T)
}

#[test]
fn test_generic_function() {
    let value: Value<i32> = Value::<i32>::VALUE(10);
    
    match value {
        Value::NONE => {
            println!("none")
        }
        Value::VALUE(value) => {
            println!("value {}", value)
        }
    }
}

struct Hi<T = SimplePerson> where T: CanSayGoodBye {
    value: T
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson { name: String::from("irfan") }
    };

    println!("{}", hi.value.name);
}

fn min<T>(value1: T, value2: T) -> T where T: PartialOrd {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_in_function() {
    let result = min(10, 20);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let point = Point{x: 10, y: 20};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> where T: PartialOrd {
    fn get_value(&self) -> &T {
        &self.x
    }
}

use core::ops::Add;

struct Apple {
    quantity: i32
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 10};

    let apple3 = apple1 + apple2;

    println!("{}", apple3.quantity);
}

fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i * 2)
    }
}

#[test]
fn test_options() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_comparing() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    println!("Apple1 == Apple2 : {}", apple1 == apple2);
    println!("Apple1 < Apple2 : {}", apple1 < apple2);
    println!("Apple1 > Apple2 : {}", apple1 > apple2);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Muhammad Irfan Bakhtiar");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Irfan", "Budi"));
    println!("{}", s.contains("Irfan"));
    println!("{}", s.starts_with("Irfan"));
    println!("{}", s.ends_with("Bakhtiar"));
    println!("{}", s.trim());
    println!("{}", &s[0..3]);
    println!("{:?}", s.get(0..3));

}

struct Category {
    id: String,
    name: String
}

use std::fmt::{Debug, Formatter};

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_format() {
    let category = Category{
        name: String::from("Gadget"),
        id: String::from("GADGET")
    };

    println!("{:?}", category);
}

#[test]
fn test_closure() {
    let sum = |value1: i32, value2: i32| -> i32 {
        value1 + value2
    };

    let result = sum(10, 10);
    println!("{}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn test_closure_as_parameter() {
    let filter = |value: String| -> String {
        value.to_uppercase()
    };

    print_with_filter(String::from("Irfan"), filter);
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_function_as_closure() {
    print_with_filter(String::from("Irfan"), to_uppercase);
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Increment");
    };

    increment();
    increment();
    increment();

    println!("Couter {}", counter);
}

struct Counter {
    counter: i32
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment");
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter{counter: 0};

    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter {}", counter.counter);
}

#[test]
fn test_vector() {
    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Muhammad"));
    names.push(String::from("Irfan"));
    names.push(String::from("Bakhtiar"));

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn tets_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Muhammad"));
    names.push_back(String::from("Irfan"));
    names.push_front(String::from("Bakhtiar"));

    for name in &names {
        println!("{}", name);
    }
}

#[test]
fn test_linked_list() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();
    names.push_back(String::from("Muhammad"));
    names.push_back(String::from("Irfan"));
    names.push_front(String::from("Bakhtiar"));

    for name in &names {
        println!("{}", name);
    }
}

#[test]
fn test_hash_map() {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(String::from("name"), String::from("Irfan"));
    map.insert(String::from("age"), String::from("25"));

    let name = map.get("name");
    let age = map.get("age");

    println!("Name {}", name.unwrap());
    println!("Age {}", age.unwrap());
}

#[test]
fn test_btree_map() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    map.insert(String::from("name"), String::from("Irfan"));
    map.insert(String::from("age"), String::from("25"));
    map.insert(String::from("country"), String::from("Indonesia"));

    for entry in map {
        println!("{} : {}", entry.0, entry.1);
    }
}

#[test]
fn test_hash_set() {
    let mut set: HashSet<String> = HashSet::new();

    set.insert(String::from("Muhammad"));
    set.insert(String::from("Muhammad"));
    set.insert(String::from("Irfan"));
    set.insert(String::from("Irfan"));
    set.insert(String::from("Bakhtiar"));
    set.insert(String::from("Bakhtiar"));

    for value in set {
        println!("{}", value);
    }
}

#[test]
fn test_btree_set() {
    let mut set: BTreeSet<String> = BTreeSet::new();

    set.insert(String::from("Muhammad"));
    set.insert(String::from("Muhammad"));
    set.insert(String::from("Irfan"));
    set.insert(String::from("Irfan"));
    set.insert(String::from("Bakhtiar"));
    set.insert(String::from("Bakhtiar"));

    for value in set {
        println!("{}", value);
    }
}

#[test]
fn test_iterator() {
    let array: [i32; 5] = [1,2,3,4,5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value);
    }

    for value in iterator {
        println!("{}", value)
    }
}

#[test]
fn test_iterator_method() {
    let vector: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    println!("{:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("{}", sum);

    let count: usize = vector.iter().count();
    println!("{}", count);

    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        None => {
            panic!("No database host provided")
        }
        Some(host) => {
            println!("Connecting to database {}", host)
        }
    }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("localhost")));
    // connect_database(None);
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err("No cache host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err("No email host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connected to application".to_string())
}

#[test]
fn test_application_error() {
    let result = connect_application(Some("localhost".to_string()));
    match result {
        Ok(host) => {println!("Success connect with message : {}", host)}
        Err(err) => {println!("Error with message : {}", err)}
    }
}

#[test]
fn test_recoverable_error() {
    // let cache = connect_cache(Some("localhost".to_string()));
    let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("Success connect to host :  {}", host)
        }
        Err(error) => {
            println!("Error with message : {}", error)
        }
    }
}

#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let x = 5;
        // r = &x; error
    }

    r = &40;

    println!("{}", r);
}

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "Irfan";
    let value2 = "Bakhtiar";
    let result = longest(value1, value2);
    println!("{}", result);
}

#[test]
fn test_lifetime_annotation_dangling_reference() {
    let string1 = String::from("Irfan");
    let string2 = String::from("Fan");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());        
    }

    println!("{}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name (&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_student() {
    let student1 = Student{
        name: "Irfan",
        last_name: "Bakhtiar"
    };
    println!("{}", student1.name);
    
    let student2 = Student {
        name: "Budi",
        last_name: "Nugraha"
    };

    let result = longest_student_name(&student1, &student2);
    println!("{}", result);

    let result = student1.longest_name(&student2);
    println!("{}", result);
}

struct Teacher<'a, ID> where ID: Ord {
    id: ID,
    name: &'a str
}

#[test]
fn test_lifetime_annotation_generic() {
    let teacher: Teacher<i32> = Teacher {
        id: 10,
        name: "irfan"
    };
    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String
}

#[test]
fn test_attribute_derive() {
    let company = Company {
        name: "Perusahaan".to_string(),
        location: "Indonesia".to_string(),
        website: "https://www.perusahaan.com".to_string()
    };

    let company2 = Company {
        name: "Perusahaan".to_string(),
        location: "Indonesia".to_string(),
        website: "https://www.perusahaan.com".to_string()
    };

    println!("{:?}", company);

    let result = company == company2;
    println!("{}", result);

    let result = company > company2;
    println!("{}", result);
}

#[test]
fn test_box() {
    let value: Box<i32> = Box::new(10);
    println!("{}", value);

    display_number(*value);
    display_number_reference(&value);
}

fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
        "Laptop".to_string(), 
        Box::new(ProductCategory::End)
    );

    println!("{:?}", category);
    print_category(&category);
}

fn print_category(category: &ProductCategory) {
    println!("{:?}", category);
}

#[test]
fn test_dereference() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result: i32 = *value1 * *value2;
    println!("{}", result);
}

struct MyValue<T> {
    value: T
}

impl<T> Deref for MyValue<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deref_srtuct() {
    let value = MyValue {value: 10};
    let real_value: i32 = *value;
    println!("{}", real_value);
}

fn say_hello_reference(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn test_deref_reference() {
    let name = MyValue {
        value: "Irfan".to_string()
    };

    say_hello_reference(&name);
}

struct Book {
    title: String
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping Book: {}", &self.title)
    }
}

#[test]
fn test_drop() {
    let book = Book{title: "Rust Programming".to_string()};
    println!("{}", book.title);
}

enum Brand {
    Of(String, Rc<Brand>),
    End
}

#[test]
fn test_multiple_ownership() {
    let apple: Rc<Brand> = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count: {}", Rc::strong_count(&apple));
    
    let laptop: Brand = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    {
        let smartphone: Brand = Brand::Of("Smartphone".to_string(), Rc::clone(&apple));
        println!("Apple reference count: {}", Rc::strong_count(&apple));
    }

    println!("Apple reference count: {}", Rc::strong_count(&apple));
}

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>
}

#[test]
fn test_ref_cell() {
    let seller = Seller {
        name: RefCell::new("Irfan".to_string()),
        active: RefCell::new(true)
    };

    {
        let mut result = seller.name.borrow_mut();
        *result = "Budi".to_string();
    }

    println!("{:?}", seller)
}

static APPLICATION: &str = "My Application";

#[test]
fn test_static() {
    println!("{}", APPLICATION);
}

static mut COUNTER: u32 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

#[test]
fn test_unsafe() {
    unsafe {
        increment();
        COUNTER += 1;
        // println!("Counter : {}", COUNTER);
    }
}

macro_rules! hi {
    () => {
        println!("Hi Macro!")
    };

    ($name: expr) => {
        println!("Hi {}", $name)
    }
}

#[test]
fn test_macro() {
    hi!();
    hi!("Irfan");
    hi! {
        "Irfan"
    }

    let name = "Irfan";
    hi!(name);
}

macro_rules! iterate {
    ($array: expr) => {
        for i in $array {
            println!("{}", i)
        }
    };

    ($($item: expr), *) => {
        $(
            println!("{}", $item);
        )*
    }
}

#[test]
fn test_macro_iterate() {
    iterate!([1,2,3,4,5,6,7,8,9,10]);
    iterate!(1,2,3,4,5,6,7,8,9,10);
}