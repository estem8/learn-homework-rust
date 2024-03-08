/*
Домашнее задание №1

Цикл for: Продажи товаров

* Дан список словарей с данными по колличеству проданных телефонов
  [
    {'product': 'iPhone 12', 'items_sold': [363, 500, 224, 358, 480, 476, 470, 216, 270, 388, 312, 186]}, 
    {'product': 'Xiaomi Mi11', 'items_sold': [317, 267, 290, 431, 211, 354, 276, 526, 141, 453, 510, 316]},
    {'product': 'Samsung Galaxy 21', 'items_sold': [343, 390, 238, 437, 214, 494, 441, 518, 212, 288, 272, 247]},
  ]
* Посчитать и вывести суммарное количество продаж для каждого товара
* Посчитать и вывести среднее количество продаж для каждого товара
* Посчитать и вывести суммарное количество продаж всех товаров
* Посчитать и вывести среднее количество продаж всех товаров
*/
use serde_json::Value;
fn main(){
  let test_data = r#"
  [
      {"product": "iPhone 12", "items_sold": [363, 500, 224, 358, 480, 476, 470, 216, 270, 388, 312, 186, 123, 12, 433]}, 
      {"product": "Xiaomi Mi 11", "items_sold": [317, 267, 290, 431, 211, 354, 276, 526, 141, 453, 510, 316]},
      {"product": "Samsung Galaxy 21", "items_sold": [343, 390, 238, 437, 214, 494, 441, 518, 212, 288, 272, 247]}
  ]
  "#;
    
    println!("Product list: \n {}", product_list(test_data));
    println!("Sales per product: \n {}", sale_per_product(test_data));
    println!("Average sales: \n {}", average_sales(test_data));
    println!("Total sales: \n {}", total_sales(test_data));
    println!("Total average sales: \n {}", total_average_sales(test_data));
}
fn total_average_sales(data:&str) -> String{
  let v:Value = match serde_json::from_str(data) {
    Ok(v) => v,
    Err(e) => panic!("Error: {}", e)
  };
  let mut total_len = 0;
  let mut total = 0;
  for item in v.as_array().unwrap(){
    total_len += item["items_sold"].as_array().unwrap().len();
    for i in item["items_sold"].as_array().unwrap(){
      total += i.as_u64().unwrap();
    }
  }
  let average = total as f32 / total_len as f32;
  format!("Average sales: {}", average)
}

fn total_sales(data: &str) -> String{
  let v:Value = match serde_json::from_str(data){
    Ok(v) => v,
    Err(e) => panic!("Error: {}", e)
  };
  let mut total = 0;
  for item in v.as_array().unwrap(){
    for i in item["items_sold"].as_array().unwrap(){
      total += i.as_u64().unwrap();
    }
  }
  format!("Total sales: {}", total)
  }




fn product_list(data: &str) -> String {
let v:Value = match serde_json::from_str(data){
  Ok(v) => v,
  Err(e) => panic!("Error: {}", e)};

let mut result: String = String::new();

for item in v.as_array().unwrap(){
  result += format!("Product: {}\n", item["product"].as_str().unwrap()).as_str();
}
result
}

fn sale_per_product(data: &str) -> String {
let mut total:String = String::new();
let v:Value = match serde_json::from_str(data){
  Ok(v) => v,
  Err(e) => panic!("Error: {}", e)
};

for item in v.as_array().unwrap(){
  let mut sum = 0;
  for i in item["items_sold"].as_array().unwrap(){
    sum += i.as_u64().unwrap();
  }
  total += format!("Product: {}, total sales: {}\n", item["product"].as_str().unwrap(), sum).as_str();
}
total
}

fn average_sales(data:&str) -> String{
  let v:Value = match serde_json::from_str(data) {
    Ok(v) => v,
    Err(e) => panic!("Error: {}", e)
  };
let mut total:u32 = 0;
for item in v.as_array().unwrap(){
  for i in item["items_sold"].as_array().unwrap(){
    total += i.as_u64().unwrap() as u32;
  }
}
let average = total as f32 / v.as_array().unwrap().len() as f32;
format!("Average sales: {}", average)
}  


