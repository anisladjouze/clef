use std::io;
use rand::{thread_rng,Rng};

fn main () {
    
    let mut rng = thread_rng() ;
    let nombre_secret : u32 = rng.gen_range(1..101);

    //saisie de l'utilisateur
	let mut input : String = String::new();
	println! ("devine mon nombre");
	println! ("saisissez votre proposition");
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().parse::<u32>().unwrap();  


    //comparaison
    if input == nombre_secret
    {
        println!("nombres egaux");
    }
    else
    {
        println!("nombres differents");
    }
}
