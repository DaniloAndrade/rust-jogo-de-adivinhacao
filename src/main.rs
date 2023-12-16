use std::cmp::Ordering;
use std::io;
use rand::{Rng};

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);
    println!("Numero secreto {}", numero_secreto);

    loop {
        println!("Digite seu palpite");
        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = palpite.trim().parse().expect("Por favor digite um numero!");
        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou, miseravi!!!");
                break
            }
        }
    }
}
