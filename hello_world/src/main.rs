fn main() {
    println!("Hello, world!");

    //first code I did <3
    let name: &str = "Austin";
    let age: i32 = 22;
    println!("I am {} and my age is {}.", name, age);

    //learning about mutatable variables
    let mut funnynum: i32 = 69;
    println!("{}", funnynum);
    funnynum = 21;
    println!("{}", funnynum);

    //You can do variales without explicitly saying their type
    let num2 = 5.699;
    println!("{}", num2);

    //character type - you HAVE to use single quotes and idk why :skull:
    let letter: char = 'A';

    // && is AND, || is OR, and ! is NOT

    //You can make a variable out of comparing to another variable
    let can_vote = age >= 18;

    println!("Can I vote? {}.", can_vote);

    //Finally an IF statement
    if can_vote {
        println!("Yes!");
    } else {
        println!("Nah lil' bro.");
    }

    //Here's a Match statement, which is kinda cool
    let day = 4;

  match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }

  //Okay time for loops

  let mut counter = 1;
  loop {
    println!("{}", counter);
    counter += 1;
    if counter == 11 {
        break println!("{}", letter);
    }
  }

  //There are also while loops

  let mut dingus = 0;
  while dingus < 6 {
    dingus += 1;
    println!("{}", dingus);
  }

  //for loops baby

  for i in 1..6 {
  println!("i is: {}", i);
}

for i in 1..=6 {
  println!("i is: {}", i);
}

//My first rust function
fn first() {
    println!("You're a loser haha!");
}

first();

//There's two types of strings, fixed and changable. The fixed str is &str, the changable string is String

//https://www.w3schools.com/rust/rust_data_structures.php
//resume here later
}
