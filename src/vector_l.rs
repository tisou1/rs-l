use std::fmt::format;
use std::collections::HashMap;
use std::hash::Hash;

pub fn hash_map () {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Red"), 20);


  // 使用元组创建map

  let teams = vec![String::from("Blue"), String::from("Red")];
  let initial_scores = vec![10, 50];

  let _scores3: HashMap<_, _> = 
    teams.into_iter().zip(initial_scores.into_iter()).collect();

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map1 = HashMap::new();
  map1.insert(field_name, field_value);
  // 这里 field_name 和 field_value 不再有效，
  // 尝试使用它们看看会出现什么编译错误！

  // i32被拷贝进map,二String被移动到map,ma鹏程为了这些值的所有者.
  // 原来的变量已经失去所有权,不能在使用这个数据
  
  // 访问hash中的值
  let key = String::from("Favorite color");
  let score2 = map1.get(&key);
  // get返回的是OPtion<T>
  println!("{:?}", score2);

  // 使用循环遍历map
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }


  let mut s = HashMap::new();
  s.insert(String::from("Blue"), 10);

  // entry方法, 如果改key有值,就不更新,如果没值才会更新
  s.entry(String::from("Yellow")).or_insert(50);
  s.entry(String::from("Blue")).or_insert(50);
  

  println!("{:?}", s);



  // 根据旧值更新新的值

  let text = "hello world wonderful world";
  let mut map5 = HashMap::new();
   
  // 将String,以空格进行拆分为数组形式
  for word in text.split_whitespace() {
    let count = map5.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map5)
}

pub fn string_l() {
  // 新建字符串
  // 创建一个空的字符串
  let mut s = String::new();

  let data = " asdasd";
  s = data.to_string();
  println!("{}", s);

  // 使用from从字符串字面值创建String

  let mut s2 = String::from("world");

  // 更新字符串
  s2.push_str(" rust");
  s2.push('c'); // 拼接单个字符

  s2 = s2 + " ddd";
  println!("{}", s2);


    let  s11 = String::from("Hello, ");
    let s22 = String::from("world!");
    let s33 = s11 + &s22; // 注意 s11 被移动了，不能继续使用

    // println!("{s11}")


    let  s3 = String::from("Hello, ");
    let s4 = String::from("world!");

    let s5 = format!("{} - {}", s3,s4);
    println!("{}",s5)

}

pub fn vl() {

  // 声明空的的vector
  let mut v: Vec<i32> = Vec::new();

  // 声明有初始值的vector
  let v2 = vec![1,2,3];

  // 添加元素
  v.push(5);
  v.push(6);

  println!("{:?}", v);


  // vector在离开其作用于也会被释放,

  // 读取

  // 返回一个引用
  let second = &v[1];

  match v.get(0) {
    Some(first) => println!("第一个值是: {}",first),
    None => println!("no value"),
  }

  if let Some(first) = v.get(0)  {
    println!("the first is {}", first)
  }

  println!("第二个元素是 {}",second );


  let mut v3 = vec![1,2,3];
  // 不可变借用
  let first = &v3[0];

  // 这里是一个可变借用
  // v3.push(6);

  // 可变借用和不可变借用不能同时存在,所以编译器会报错
  println!("第一个值是{first}");


  /*
  在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
   */


   // 遍历vector中的元素

   let v4 = vec![1,23,3,45,6];
   // 遍历不可变引用
   for i in &v4 {
    println!("{i}");
   }

   let mut v5 = vec![1,23,3,45,6];
   // 遍历可变引用
   for i in &mut v5 {
      *i += 50;
   }
   println!("{:?}", v5);


   // 使用枚举存储多种类型
   #[derive(Debug)]
   enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
   }

   let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(4.2),
    SpreadsheetCell::Text(String::from("blue")),
   ];

   
   println!("{:#?}", row)

}