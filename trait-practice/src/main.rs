fn main() {
    trait_example();
    trait_example2();
    trait_example3();
    trait_example4();
}

fn trait_example() {
    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    print_area(&rect);
    print_length(&rect);

    let tri = RightTriangle {
        base: 10.0,
        height: 20.0,
    };

    print_area(&tri);
    print_length(&tri);

    let line = Line {
        x1: 0.0,
        y1: 0.0,
        x2: 10.0,
        y2: 10.0,
    };

    print_length(&line);
    // print_area(&line); // Error: the trait `CalcArea` is not implemented for `Line`
}

fn print_area<T: CalcArea>(shape: &T) {
    println!("The area is: {}", shape.calc_area());
}

trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTriangle {
    base: f64,
    height: f64,
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

trait CalcLength {
    fn calc_length(&self) -> f64;
}

fn print_length<T: CalcLength>(shape: &T) {
    println!("The length is: {}", shape.calc_length());
}

struct Line {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        ((self.x2 - self.x1).powi(2) + (self.y2 - self.y1).powi(2)).sqrt()
    }
}

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.base + self.height + (self.base.powi(2) + self.height.powi(2)).sqrt()
    }
}

trait HelloWorld {
    // Default implementation
    fn hello_world(&self) {
        println!("Hello, World!");
    }
}

struct Dummy;

impl HelloWorld for Dummy {}

struct Dummy2;

impl HelloWorld for Dummy2 {
    // Overriding the default implementation
    fn hello_world(&self) {
        println!("Hello, World! from Dummy2");
    }
}

fn trait_example2() {
    let dummy = Dummy;
    dummy.hello_world();

    let dummy2 = Dummy2;
    dummy2.hello_world();
}

use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn length(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        // same length
        let self_len = self.length();
        let other_len = other.length();
        self_len.eq(&other_len)
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // compare length
        let self_len = self.length();
        let other_len = other.length();
        self_len.partial_cmp(&other_len)
    }
}

fn trait_example3() {
    let p1 = Point2D { x: 1.0, y: 1.0 };
    let p2 = Point2D { x: 2.0, y: 2.0 };

    let p3 = p1 + p2;
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p1 + p2 : {:?}", p3);

    let p4 = Point2D { x: 1.0, y: 1.0 };
    let p5 = Point2D { x: 1.0, y: 1.0 };
    println!("p4: {:?}", p4);
    println!("p5: {:?}", p5);
    println!("p4 == p5? : {}", p4 == p5);

    let p6 = Point2D { x: 1.0, y: 1.0 };
    let p7 = Point2D { x: 2.0, y: 2.0 };
    println!("p6: {:?}", p6);
    println!("p7: {:?}", p7);
    println!("p6 < p7? : {}", p6 < p7);
}

trait SayHello {
    fn say_hello(&self);
}

trait SayThankyou {
    fn say_thank_you(&self);
}

struct EnglishPerson;
struct SpanishPerson;

impl SayHello for EnglishPerson {
    fn say_hello(&self) {
        println!("Hello!")
    }
}

impl SayThankyou for EnglishPerson {
    fn say_thank_you(&self) {
        println!("Thank you!")
    }
}

impl SayHello for SpanishPerson {
    fn say_hello(&self) {
        println!("Hola")
    }
}

impl SayThankyou for SpanishPerson {
    fn say_thank_you(&self) {
        println!("Gracias")
    }
}

trait Run {
    fn run(&self);
}

impl Run for EnglishPerson {
    fn run(&self) {
        println!("Run!!!")
    }
}

impl Run for SpanishPerson {
    fn run(&self) {
        println!("Correr!!!")
    }
}

fn say_hello_general<T: SayHello>(speaker: &T) {
    speaker.say_hello();
}

fn say_thank_you_general<T: SayThankyou>(speaker: &T) {
    speaker.say_thank_you();
}

fn say_thank_you_and_run<T: SayThankyou + Run>(person: &T) {
    person.say_thank_you();
    person.run();
}

fn trait_example4() {
    let en = EnglishPerson;
    let sp = SpanishPerson;

    say_hello_general(&en);
    say_thank_you_general(&en);

    say_hello_general(&sp);
    say_thank_you_general(&sp);

    say_thank_you_and_run(&en);
    say_thank_you_and_run(&sp);
}
