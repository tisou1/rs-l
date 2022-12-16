pub fn trait_bound() {
  //impl Trait 语法适用于直观的例子，它实际上是一种较长形式语法的语法糖。我们称为 trait bound，
  pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
  }

  // trait bound适用于更复杂的场景,
  // 如下获取两个实现了Summary的参数,用impl  trait写法
  pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
  }
  // 使用trait bound写法
  pub fn notify3<T: Summary>(item: &T, item2: &T){}

  // 通过+ 号实现多个trait
  // impl trait语法
  pub fn notify4(item:(impl Summary + Display)) {}
  // trait bound语法
  pub fn notify5<T: Summary + Display>(item: &T) {}



  // 使用where简化trait bound
//   fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

//     fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {
  
}
pub fn trait_p() {
  // 使用trait接受多种不同类型的参数
  pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
  }

  pub struct Person {
    pub name: String,
    pub age: i32,
  }

  impl Summary for Person {
      fn summarize(&self) -> String {
          format!("Person name: {}", self.name)
      }
  }
  let tisou1 = Person {
    name: String::from("tisou1"),
    age: 22
  };

  // tisou1 实现了train Summary,所以这里作为参数传递给notify
  notify(&tisou1)
}
 // 定义trait
 pub trait Summary {
  fn summarize(&self) -> String;
}

pub trait  Display {
    fn get_name(&self) -> String;
}

pub fn trait_l(){
 

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }

  // 结构体NewsArticle实现了trait的summarize方法
  impl Summary for NewsArticle{
      fn summarize(&self) -> String {
          format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
  }

  pub struct  Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
  }

  impl Summary for Tweet {
      fn summarize(&self) -> String {
          format!("{}: {}", self.username, self.content)
      }
  }
}