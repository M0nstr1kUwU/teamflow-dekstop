fn main() {
    let app_name: &str = "TeamFlow-dekstop";
    println!("Приложение {app_name}");
    // Переменные в Rust по умолчанию неизменяемы
    let name: &str = "Oleg";
    let greeting: &str = &build_greeting(name);
    println!("{greeting}");

    let title: &str = "Сделать первый экран дикий KanBan";
    let assignee: &str = "Titas";
    let score: u32 = 11;

    // let validation_result: Result<String, String> = validate_task_title(title);
    // match validation_result {
    //     Ok(clean_title) => {
    //         println!("Задача создана: {clean_title}");
    //     }
    //     Err(error_message) => {
    //         println!("Ошибка: {error_message}");
    //     }
    // }

    let validation_result: Result<String, String> = format_task_summary(title, assignee, score);
    match validation_result {
        Ok(clean_title) => {
            println!("Задача создана: {clean_title}");
        }
        Err(error_message) => {
            println!("[!] Ошибка: {error_message}");
        }
    }
}

// fn validate_task_title(title: &str) -> Result<String, String> {
//     if title.trim().is_empty() {
//         return Err("Название задачи не может быть пустым!".to_string());
//     }
//     Ok(title.trim().to_string())
// }

fn format_task_summary(title: &str, assignee: &str, score: u32) -> Result<String, String> {
    if title.trim().is_empty() {
        return Err("[!] Название задачи не может быть пустым!".to_string());
    } else if assignee.trim().is_empty() {
        return Err("[!] Исполнитель не может быть пустым!".to_string());
    }
    let result: String = format!("Задача: {title} | Исполнитель {assignee} | Вес: {score}");
    Ok(result)
}

fn build_greeting(user_name: &str) -> String {
    let message: String = format!("Привет, {user_name}, добро пожаловать в моё первое приложение");
    // format подготавливает строку, но не печатает её сразу
    message
    // Без точки с запятой строка - это возвратная строка (return)
}
