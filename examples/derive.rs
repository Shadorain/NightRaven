#![allow(dead_code)]

use nightraven::NightRaven;

enum SecondLevel {
    SomeVariant { field1: bool, field2: u32 },
    TestI,
    TestII,
}
#[derive(NightRaven)]
enum TopLevel {
    SubVariant(SecondLevel),
    Test1,
    Test2,
}

fn main() {
    println!("List: {:?}", TopLevel::Test1.list_names());
    // println!("Concatenated names: {}", TopLevel::SubVariant.concatenated_names());
}
