mod module1;
mod vector_l;
mod sort_l;
mod panic_l;
mod flood_type;
mod trait_l;

// module2 是module1的子模块,这里使用use,是方便后面进行路径的简写
use crate::module1::module2;
use crate::sort_l::sort;
// vector在根路径下,不需要再使用use,
// use crate::vector_l;

fn main() {
    // println!("Hello, world!");
    // 模块拆分引用和调用.
    // module1::vertor_ll();
    // module2::vecc();
//    vector_l::vl()

    // vector_l::string_l();

    // let mut arr = [3, 2, 1, 4, 5];
    // sort::quick_sort(&mut arr);

    // assert_eq!(arr, [1, 2, 3, 4, 5]);
    // println!("{:?}", arr)


    // vector_l::hash_map();

    // panic_l::fc();

    // 泛型
    // flood_type::flood_l();


    trait_l::trait_p();
    //.. 
}
