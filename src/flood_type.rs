

pub fn flood_l() {
  let number_list = vec![1,2,3,4,5,6,6];
  let lar = largest_t(&number_list);
  println!("最大值是: {}", lar)
}

fn largest(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &number in list {
    if largest < number {
      largest = number
    }
  }
  largest
}

// 使用泛型定义函数
fn largest_t<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

// 结构体泛型

struct  Point<T> {
  x: T,
  y: T,
}
// 定义多个泛型参数,实现x,y不同类型
struct  Point2<T, U> {
  x: T,
  y: U,
}

fn struct_flood() {
  let integer = Point {x:5, y: 10};
  let float = Point {x: 1.0, y: 3.0};

  let integer_float = Point2{ x: 3, y: 5.4};

}

// 枚举中使用泛型

fn enum_flood() {
  enum Option<T> {
      Some(T),
      None,
  }

  enum  result<T, E> {
      Ok(T),
      Err(E),
  }
}


// 方法中使用泛型
fn fn_flood() {
  struct  Point3<T> {
    x: T,
    y: T
  }
  
  impl<T> Point3<T>{
    fn x(&self) -> &T {
      &self.x
    }
  }
  let p3 = Point3{x: 5, y: 10};
  println!("p3.x = {}", p3.x());


  // 为特定类型定义方法
  impl Point3<f32>{
   fn cc(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
   }
  }
}