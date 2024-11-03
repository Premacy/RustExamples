extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадайте число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Секретное число {}", secret_number);
    
    loop{
        println!("Пожалуйста, введите предположение!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess) // возвращает объект Result с методами ok, expect
            .ok()
            .expect("Не удалось прочитать строку"); // выбросит panic в случае ошибки

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("Ваша попытка {}", guess);

        match guess.cmp(&secret_number) { // cmp возвращает тип Ordering(перечисление)
            Ordering::Less => println!("Слишком маленькое!"), 
            Ordering::Greater => println!("Слишком большое!"), 
            Ordering::Equal => {
                 println!("Вы выиграли!");
                 break;
            } // match позволяет привязать ветку кода к каждому типу перечисления
        }
    }
}
