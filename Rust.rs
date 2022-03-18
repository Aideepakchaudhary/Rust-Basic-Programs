/*
fn main() {

    let charater = ['a', 'b', 'c']; // array..

    let number: [i32;5];
    number = [5;5];
    println!("num is {}", number[0]);
    let ch = charater[0];
    println!("first character is {}", ch);
     */
    /*

    // tuple
    let stuff = (10,3.14, 'x');
    let first_item = stuff.2;
    println!("First_item is {}", first_item);


}*/
/*

fn main() {
let celius = 23.0;
    let farenheit_temp = celsius_to_farenheit(celius);

    assert_eq!(farenheit_temp, 73.4);
    println!("Test Passed!");
}
fn celsius_to_farenheit(t : f64) -> f64
{
    let f = (1.8 * t) + 32 as f64;
    return f;
}
*/

/*

fn main()
{
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max: i32;
    let mut min = numbers[0];
    let mut mean : f64;

    let mut maxi = 0;
    let mut sum = 0;
    for &i in numbers.iter() {
        sum += i;
        if i > maxi {
            maxi = i;
        }
        if i < min {
            min = i;
        }

    }
    mean = sum as f64 / (numbers.len() as f64);

    assert_eq!(maxi, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("test passed");
}

 */
/*
fn main()
{
    let mut test1 = "We need more space. ";
    // trim_space(&test1);
    assert_eq!(trim_space(test1), "We need more space.");
    println!("TestCase Passed !!")
}


fn trim_space(s : &str) -> &str {
    let mut start = 0;
    let mut end = 0;
    for (index, character) in s.chars().enumerate(){
        if(character != ' ') {
            start = index;
            break;
        }
    }

    for (index, character) in s.chars().rev().enumerate() {
        if(character != ' ') {
            end = s.len() - index;
            break;
        }
    }

    return &s[start..end];

}

 */

/*
fn main() {
    let number = [2,4,6,8,10];

    for (i,no) in number.iter().enumerate() {
        println!("{}: {}", i, no);
    }
}

 */
/*

use std::io;

fn main()
{
    let mut s = String::new();
    println!("Enter a message: ");

    io::stdin().read_line(&mut s);
    println!("buffer is {}", s);
}

 */

/*

// Struct

#[derive(Debug)]
struct Rectangle {
    height: f64,
    width: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }
    fn scale(&mut self, val: f64) {
        self.width *= val;
        self.height *= val;
    }
}

fn main()
{
    let mut rect = Rectangle {
        height: 1.2,
        width: 3.4
    };

    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);

    assert_eq!(rect.get_area(), 1.02);
    println!("Test passed!");
}

 */

/*

// traits

use std::fmt::format;

struct Satellite {
    name: String,
    velocity: f64
}

trait Description {
    fn describe(&self) -> String;
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second!", self.name, self.velocity)
    }
}

fn main()
{
    let hubble = Satellite {
        name: String::from("Hubble Telelscope"),
        velocity: 4.72
    };
    println!("Hubble name is {} and velocity is {}", hubble.name, hubble.velocity);
}

 */
