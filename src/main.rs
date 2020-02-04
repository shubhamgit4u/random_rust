extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    let random = rand::thread_rng().gen_range(0,9);
    println!("let's both guess number ");
    let  mut guess = String::new();
    println!("enter your guess :");
     std::io::stdin().read_line(&mut guess).unwrap();
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    match guess.cmp(&random){
       Ordering::Less=>  println!("you guess less then actual one ,number was {}",random),
        Ordering::Equal=>   println!("Winner Winner chicken dinner!!!!I also choosed {}",random),
         Ordering::Greater => println!("you guess above then actual number, mine was {}",random),
    }
}