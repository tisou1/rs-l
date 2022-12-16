use core::panic;
use std::{fs::File, io::ErrorKind};



pub fn fc() {

  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      // 如果是因为不存在而报错, 则会创建该文件,
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
          panic!("Problem opening the file: {:?}", other_error)
        }
    }
  };
}