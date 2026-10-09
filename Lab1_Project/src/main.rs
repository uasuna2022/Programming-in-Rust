use rand::RngExt;
use std::fs::File;
use std::io::Write;

fn generate_10_powers(x: u64) -> [u64; 10] {
    let mut powers = [0; 10];
    let mut number = 1;

    /* for i in 0..powers.len() {
        powers[i] = number;
        number *= x;
    } */

    for item in &mut powers {
        *item = number;
        number *= x;
    }

    powers
}

fn check_collatz_hypothesis(tab: [u64; 10]) -> [bool; 10] {
    let mut results = [false; 10];
    for i in 0..tab.len() {
        let mut n = tab[i];
        let mut is_true = false;
        for _ in 0..100 {
            if n == 1 || n == 2 || n == 4 {
                is_true = true;
                break;
            }
            else if n.is_multiple_of(2) {
                n /= 2;
            }
            else {
                n = 3 * n + 1;
            }
        }

        results[i] = is_true;
    }
    results
}
fn sum_digits (mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
fn generate_number_info (x: u32) -> (u32, bool, String) {
    let is_even = x.is_multiple_of(19);
    let fun_fact = format!("Suma cyfr liczby {} wynosi {}", 
        x, sum_digits(x));

    (x, is_even, fun_fact)
}

fn main() {
    println!("Hello, world!");

    let wynik = loop {
        println!("Podaj liczbę: ");
        let mut input = String::new();
        let result = std::io::stdin().read_line(&mut input);
        if let Err(e) = result {
            println!("Błąd odczytu danych: {e}");
            break true;
        }
        let x = input.trim().parse::<u64>();
        match x {
            Err(e) => {
                println!("Błąd parsowania liczby: {}", e);
                break true; 
            }
            Ok(n) => {
                if n == 0 {
                    println!("Wpisana liczba jest zerem");
                    break false;
                }
                else {
                    let x = n + rand::rng().random_range(0..=5);
                    println!("Nowa wartość x: {x}");

                    let powers = generate_10_powers(x);
                    println!("Potęgi x: {powers:?}");

                    let results = check_collatz_hypothesis(powers);
                    println!("Wyniki: {results:?}");

                    let mut file = File::create("xyz.txt")
                        .expect("Nie można utworzyć pliku");
                    /*
                        let mut file = match File::create("xyz.txt") {
                            Ok(f) => f,
                            Err(e) => {
                                panic!("Nie można utworzyć pliku: {e}");   
                            }
                        }
                     */
                    let data = format!("{results:?}\n");
                    file.write_all(data.as_bytes())
                        .expect("Nie udało się zapisać danych do pliku");
                    println!("Zapisano wyniki do pliku xyz.txt");
                }
            }
        }
    };
    
    println!("Wynik pętli: {wynik}");
    match wynik {
        true => println!("Pętla zakończona z powodu błędu."),
        false => println!("Pętla zakończona z powodu wpisania liczby 0 przez użytkownika."),
    }


    'main_loop: for i in 37..67 {
        'inner_loop: for j in 2..5 {
            let thriplet: (u32, bool, String) = generate_number_info(i + j);
            println!("Liczby {} i {} dają wynik: {}. Czy suma jest podzielna przez 19?",
                i, j, thriplet.0);
            match thriplet.1 {
                true => {
                    println!("Tak! Kończę pętlę!");
                    break 'main_loop;
                }
                false => {
                    println!("Nie! Kontynuuję pętlę!");
                    println!("Fun fact: {}", thriplet.2);
                    continue 'inner_loop;
                }
            }
        }
    }

    let (num, is_div_by_19, fun_fact) = generate_number_info(2137);
    println!("Liczba: {}, Czy jest podzielna przez 19: {}, Fakt: {}", num, is_div_by_19, fun_fact);

}
