use std::fmt::Display;

// 生命周期
pub fn life_fn() {
  // rust中每一个引用都有其生命周期(lefitime), 也就是引用保持有效的作用域
  // 当有多种可能类型的时候必须要注明类型,也会出现引用的生命周期以一些不同的方式
  // 相关联的情况,所以rust需要我们使用泛型生命周期参数来注明他们的关系

  // 生命周期的主要目的是避免悬挂引用

  {
    let r;
     {
      let x = 5;
      r = &5;
     }
     println!("r: {}", r)
  }

  // 上面代码,编译之后有问题,
  // 在内部作用域中声明了x,在其作用域结束后,会清除x的值
  // r又获得了x的引用,在外部作用域中,会打印r,这就会有问题

  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = get_max_len(string1.as_str(), string2);
  println!("The longest string is {}", result);

  // 一个存放引用的结构体, 之前都是创建存放有所有权的数据
  let novel = String::from("call me Ishmael, some years aho...");
  let first_sentence = novel.split('.').next().expect("could not find a '.'")
  let i = ImportantExcerpt {
    part: first_sentence,
  };
}


/*
  rust不清楚要返回的引用是指向x或y,事实上我们也不知道,因为函数体中if快返回一个x
  而 else快返回一个y的引用

  所以我们需要增加泛型生命周期参数来定义引用之间的关系以便借用检查器可以进行分析
*/
// fn get_max_len(x: &str, y: &str) -> &str {
//   if x.len() > y.len() {
//     x
//   } {
//     y
//   }
// }

/*
现在函数签名表明对于某些生命周期 'a，函数会获取两个参数，他们都是与生命周期
'a 存在的一样长的字符串 slice。函数会返回一个同样也与生命周期 'a 
存在的一样长的字符串 slice。
它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。这些关系就是我们希望 Rust 分析代码时所使用的。

 */
// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分
// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
fn get_max_len<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// 返回值的生命周期与参数完全没有关联
// 这种情况最好返回一个有所有权的数据类型,而不是一个引用
// fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
//   let result = String::from("really long string");
//   result.as_str()
// }



// 结构体定义中的生命周期注解
// 表明ImportantExcerpt实例不能比其part字段中的引用存在更久
struct  ImportantExcerpt<'a> {
  part: &'a str
}


// 现在我们已经知道了每一个引用都有一个生命周期，而且我们需要为那些使用了引用的函数或结构体指定生命周期。


// 方法定义中的生命周期注解
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
      3
    }
}

// 静态生命周期
// 'static，其生命周期能够存活于整个程序期间
// let s: &'static str = "I have a static lefetime"


// 结合泛型类型参数、trait bounds 和生命周期
fn longest_with_an_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T,
) -> &'a str 
where T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}