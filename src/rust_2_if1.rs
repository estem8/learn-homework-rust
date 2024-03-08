/*
Домашнее задание №1

Условный оператор: Сравнение строк

* Написать функцию, которая принимает на вход две строки
* Проверить, является ли то, что передано функции, строками. 
  Если нет - вернуть 0
* Если строки одинаковые, вернуть 1
* Если строки разные и первая длиннее, вернуть 2
* Если строки разные и вторая строка 'learn', возвращает 3
* Вызвать функцию несколько раз, передавая ей разные праметры 
  и выводя на экран результаты
*/
pub fn main(){
  let first = input();
  let second = input();
  println!("{}", compare(first, second));
}

fn compare(first: String, second: String) -> u8 {
  if first == second {
    return 1;
  } else if first.len() > second.len() {
    return 2;
  } else if second.trim() == "learn" {
    return 3;
  } else {
   return 0;   
  }
}
fn input() -> String{
  let mut result = String::new();
  println!("Enter your string: ");
  std::io::stdin().read_line(&mut result).expect("Failed to read line");
    result
  }
  