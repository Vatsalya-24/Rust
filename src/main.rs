use rand ::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;

fn main(){
    println!{"Welcome to the guessing game"};
    let secret_number =  rand::thread_rng().gen_range(1..101);
    println!{"Scret number is:{}",secret_number}
    loop{

        println!("Please input your Guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("Failed top read");
        println!("your guessed:{}",guess);
        let guess:u32 = guess.trim().parse().expect("type an integer");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
        },
}
    
        }
    }

