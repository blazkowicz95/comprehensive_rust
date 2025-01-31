use std::io;
use rand::Rng;
fn main(){
    
    let mut rng = rand::rng();
    let rand_num: u16 = rng.random_range(1..101);
    loop {
        let mut input_txt = String::new();
        let _ = io::stdin().read_line(&mut input_txt);
        let num:u16 = match input_txt.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        if rand_num == num {
            println!("Correct Guess: {}",num);
            break;
        }
        if rand_num > num {
            println!("InCorrect Guess, too low: {}",num);
        }
        if rand_num < num {
            println!("Incorrect Guess, too high: {}", num)
        }
        

    }
}