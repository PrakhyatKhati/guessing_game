use std::io; 
use std::cmp::Ordering;

use rand::Rng;
// the above step is to bring the type into scope from the standard library.


fn main() {
    // println!("Guess the number");
    let secret_numeber = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is :{secret_numeber}");
    

   loop{ println!("Please input your guess");
        let mut guess = String::new(); // making the variable mutuable
    // as new value will be stored in this variable.
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32= match guess.trim().parse(){Ok(num)=>num,Err(_)=>continue,};
        //println!("Please input your guess");
        println!("You guessed : {guess}");


        // match is an expression to decide what to do next based on what was returend.
        match guess.cmp(&secret_numeber){
            Ordering::Less => println!("Too small !!"),
            Ordering::Greater =>println!("Too big"),
            Ordering::Equal => {println!("You win !!");
                                break;}
            // here we are adding the break to break after right guess.
    }
}



}
