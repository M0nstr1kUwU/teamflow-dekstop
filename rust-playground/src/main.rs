// https://doc.rust-lang.org/book/ch03-05-control-flow.html
use std::io::{stdin, stdout, Write};    // stdin — для чтения ввода из консоли, stdout — для вывода в консоль, Write — нужен для метода flush()

fn main() {
    clear_console();
    print!("Name> ");
    stdout().flush().unwrap();      // flush() принудительно выводит текст в консоль сразу
    let mut user_name = String::new();
    stdin()
        .read_line(&mut user_name)
        .expect("[!] Не удалось прочитать строку");
    let c_un: &str = user_name.trim();
    build_greeting(c_un);
    wait_enter();


    let app_name: &str = "TeamFlow-desktop";    // &str — строковый срез/ссылка на строку
    print_menu(app_name);

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("[!] Не удалось прочитать строку");
        match input.trim() {
            "1" => {
                clear_console();
                char_fill(30, '=', format_task_summary);
                wait_enter();
            },
            "2" => {
                clear_console();
                char_fill(30, '=', validate_task_title);
                wait_enter();
            },
            "0" => break,
            _ => println!("[!] Неизвестная команда")
        }
    }
}

fn validate_task_title() {
    print!("Title> ");
    stdout().flush().unwrap();
    let mut title = String::new();
    stdin()
        .read_line(&mut title)
        .expect("[!] Не удалось прочитать строку");

    let clean_title = title.trim();

    if clean_title.is_empty() {
        println!("[!] Название задачи не может быть пустым!");
        return;
    }

    println!("Задача создана: {title}");
}

fn format_task_summary() {
    print!("Title> ");
    stdout().flush().unwrap();
    let mut title = String::new();
    stdin()
        .read_line(&mut title)
        .expect("[!] Не удалось прочитать строку");

    print!("Assignee> ");
    stdout().flush().unwrap();
    let mut assignee = String::new();
    stdin()
        .read_line(&mut assignee)
        .expect("[!] Не удалось прочитать строку");

    print!("Score> ");
    stdout().flush().unwrap();
    let mut score = String::new();
    stdin()
        .read_line(&mut score)
        .expect("[!] Не удалось прочитать строку");

    let clean_title = title.trim();
    let clean_assignee = assignee.trim();
    let clean_score = score.trim();

    if clean_title.is_empty() {
        println!("[!] Название задачи не может быть пустым!");
        return;
    }
    if clean_assignee.is_empty() {
        println!("[!] Исполнитель не может быть пустым!");
        return;
    }
    let clean_score: u32 = match clean_score.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("[!] Вес задачи должен быть числом!");
            return;
        }
    };
    println!("Задача: {clean_title} | Исполнитель: {clean_assignee} | Вес: {clean_score}");
}

fn clear_console() {    // Это ANSI escape-код:
    print!("\x1B[2J\x1B[1;1H");     // \x1B[2J — очистить экран, \x1B[1;1H — переместить курсор в левый верхний угол
    stdout().flush().unwrap();
}


fn wait_enter() {   // Функция-пауза и всё
    let mut pause = String::new();
    stdin()
        .read_line(&mut pause)
        .expect("");
}

fn print_menu(app_name: &str) {
    char_main_fill(30, '~');
    println!(
    "
    <{app_name}>
    1. Format Task Summary
    2. Validate Task Title
    0. Exit
    "
    );
    char_main_fill(30, '~');
}

fn build_greeting(user_name: &str) {
    println!("Привет, {user_name}, добро пожаловать в моё первое приложение!");
}

fn char_fill<F>(num: usize, symbol: char, func: F)      // F — тип функции
where
    F: FnOnce(),    // FnOnce() значит: функция не принимает аргументы и может быть вызвана один раз
{
    for _ in 0..num {
        print!("{symbol}");
    }
    println!();
    func();     // Вызываем переданную функцию
    for _ in 0..num {
        print!("{symbol}");
    }
    println!();
}

fn char_main_fill(num: usize, symbol: char) {
    for _ in 0..num {
        print!("{symbol}");
    }
    println!();
}