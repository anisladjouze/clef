use std::io;
use rand::{thread_rng,Rng};
use std::cmp::Ordering;

fn main () {
    
    let mut rng = thread_rng() ;
    let nombre_secret : u32 = rng.gen_range(1..101);

    println! ("devine mon nombre");
    println! ("saisissez votre proposition");

    loop
    {
        //saisie de l'utilisateur
        let mut input : String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse::<u32>().unwrap();  


        //comparaison
        let result = input.cmp(& nombre_secret);
        match result {
            Ordering::Less => {
                println!( "la proposition est trop petite");
            }
            Ordering::Equal => { 
                println! (" c'est le bon nombre");
                break;
            }
            Ordering::Greater => {
                println!("la proposition est trop grande");
            }
        }
    }   
}
