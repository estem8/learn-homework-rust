/*
Домашнее задание №1
Условный оператор: Возраст
* Попросить пользователя ввести возраст при помощи input и положить 
  результат в переменную
* Написать функцию, которая по возрасту определит, чем должен заниматься пользователь: 
  учиться в детском саду, школе, ВУЗе или работать
* Вызвать функцию, передав ей возраст пользователя и положить результат 
  работы функции в переменную
* Вывести содержимое переменной на экран
*/

pub fn main(){
  let age = input();
  let result = age_check(age);
  println!("{}", result);
}

fn input() -> String{
  let mut age = String::new();
  println!("Enter your age: ");
  std::io::stdin().read_line(&mut age).expect("Failed to read line");
  age
}

fn age_check(age: String) -> String {
  let age = age.trim().parse::<i32>().unwrap();
  let result: String;
  match age {
      0..=6 => result = String::from("Go to Kindergarten"),
      7..=17 => result = String::from("Go to School"),
      18..=22 => result = String::from("Go to University"),
      23..=75 => result = String::from("Go to Work"),
      _ => result = String::from("Invalid age"),
  }
  result
}

