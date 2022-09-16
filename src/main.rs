use std::io;

fn main() {
    
    loop {
        menu();

        let opcao = ler_int();

        match opcao {
            1 => println!("beijinho*"),
            2 => ouvir_caio("Adoro anarcocapitalismo"),
            3 => ouvir_caio("legal, namorado Caio"),
            4 => ouvir_caio("Ok, amor, mas o que é AUR?"),
            5 => break,
            _ => print!(""),
        }
    }
}

fn menu() {
    println!("\nO que desejas que eu faças, amor Caio?
    1 - Beijo
    2 - Falar sobre anarcocapitalismo
    3 - Pergunte: 'Como está o preço do bitcoin hoje?'
    4 - Diga: 'Me ensine a como usar o Linux'
    5 - Matar a namorada");
}

fn ler_int() -> u8 {
    let mut t = String::new();

    io::stdin()
        .read_line(&mut t)
        .unwrap();

    t.trim().parse::<u8>().unwrap()
}

fn ouvir_caio(resposta: &str) {
     let mut t = String::new();
 
    println!("Digite o que você irá falar pra ela:");

      io::stdin()
          .read_line(&mut t)
          .unwrap();

      println!("{}", resposta);
 }
