use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct NumberInto {
    value: i32,
}

impl Into<NumberInto> for i32 {
    fn into(self) -> NumberInto {
        NumberInto { value: self }
    }
}

#[derive(Debug)]
struct NumberFrom {
    value: i32,
}

impl From<i32> for NumberFrom {
    fn from(item: i32) -> Self {
        NumberFrom { value: item }
    }
}

fn main() {

    let num = NumberFrom::from(30);
    println!("My number is {:?}", num);


    let int = 5;
    // Try removing the type annotation
    let num: NumberInto = int.into();
    println!("My number is {:?}", num);


    let int = 3;
    // do not need to provide an implementation for both traits
    // use `Into`
    let num: NumberFrom = int.into();
    println!("My number is {:?}", num);

    let num = NumberInto::from(NumberInto { value: 6 });
    println!("My number is {:?}", num);

}