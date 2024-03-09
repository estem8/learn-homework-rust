/*
Домашнее задание №1
Цикл while: hello_user
* Напишите функцию hello_user(), которая с помощью функции input() спрашивает 
  пользователя “Как дела?”, пока он не ответит “Хорошо”
*/

use std::io::Write;

pub fn main(){
  hello_user();
}
fn hello_user(){
    let mut answer = String::new();
    loop {
      println!("Как дела?");
      std::io::stdout().flush().unwrap();
      std::io::stdin().read_line(&mut answer).expect("Failed to read line");
      println!("{:#?}", answer.trim_start().trim_end());
      if answer.trim() == "Хорошо" {
        break
      } else {
        answer.clear()
      }
      
    }
}