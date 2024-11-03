use std::thread;
use std::sync::{Mutex, Arc};

struct Table{
    forks : Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize
}  

// В impl мы размещаем реализацию методов, которые относятся к типу Philosopher
impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher{
            name: name.to_string(),
            left: left,
            right: right
        } // Отсутствует ; Rust - язык, основанный на выражениях. Тут мы возващаем последнее выражение(результат его вычисления)
    }

    fn eat(&self, table: &Table){
        // Стоит отметить, что мьютексы в Rust ведут себя как lock_guard в С++
        // они автоматически освобождают мьютекс при выходе из области видимости

        let _left = table.forks[self.left].lock().unwrap(); // расписать про unwrap
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} начал(а) есть", self.name);
        
        thread::sleep_ms(1000);

        println!("{} закончил(а) есть", self.name);
    }
}

fn main() {
     // Arc<T> - атомарный счетчик ссылок. ++ при передаче в новый поток, -- при выходе
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()), 
        Mutex::new(()), 
        Mutex::new(()), 
        Mutex::new(()),
        ]});
    
    // Вектор он и в Африке вектор. Только тут макрос...
    let philosophers = vec![ 
        Philosopher::new("Джудит Батлер", 0, 1), 
        Philosopher::new("Рая Дунаевская", 1, 2), 
        Philosopher::new("Зарубина Наталья", 2, 3),
        Philosopher::new("Эмма Гольдман", 3, 4), 
        Philosopher::new("Анна Шмидт", 0, 4),
    ];
        
    /*for p in &philosophers {
        p.eat();
    }*/

    // Тут мы создаем вектор дескрипторов, который содержит дескрипторы созданных
    // потоков, в которых едят наши философы
    let handlers : Vec<_> = philosophers.into_iter().map(|p| { // создаем итератор для каждого элемента и вызываем для него лямбду(замыкание)
        let table = table.clone();

        thread::spawn(move || { // создание потока. move == mutable лямбда в с++
            p.eat(&table);
        })
    }).collect(); // Создание вектора типа Vec<_> с дескрипторами потоков.

    for h in handlers {
        h.join();
    }
}
