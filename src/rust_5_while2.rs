/*
Домашнее задание №1
Цикл while: ask_user со словарём
* Создайте словарь типа "вопрос": "ответ", например:
  {"Как дела": "Хорошо!", "Что делаешь?": "Программирую"} и так далее
* Напишите функцию ask_user() которая с помощью функции input()
  просит пользователя ввести вопрос, а затем, если вопрос есть
  в словаре, программа давала ему соотвествующий ответ. Например:
    Пользователь: Что делаешь?
    Программа: Программирую
*/

use std::collections::HashMap;

pub fn main(){
  ask_user();
}

fn ask_user(){
    let quiz = HashMap::from([
        ("Как дела", "Good"),
        ("Что делаешь?", "Coding"),
        ("Как делаешь?", "Lazy"),
    ]);

    let mut answer:String = String::new();
    loop {
        println!("Enter question");
        std::io::stdin().read_line(&mut answer).expect("Failed to read line");
        answer = answer.trim_start().trim_end().to_string();
        if quiz.contains_key(answer.as_str()) {
            println!("{:?}", quiz.get(answer.as_str()).unwrap());
        } else if answer == "stop" {
            break
        }
        answer.clear();
    }        
}