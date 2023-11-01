#![allow(dead_code)]

use nightraven::nightraven;

nightraven! {
    enum TopLevel {
        SubVariant(
            enum SecondLevel {
                SubVariant(
                    #[derive(Debug)]
                    enum ThirdLevel {
                        SomeVariant { field1: bool, field2: u32 },
                        TestI,
                        TestII,
                    }
                ),
                TestA,
            }
        ),
        Test1,
        Test2,
    }
}

// nightraven! {
//     enum TopLevel1 {
//         SubVariant(
//             #[derive(Debug)]
//             enum SecondLevel1 {
//                 TestA,
//                 TestB,
//             }
//         ),
//         Test1,
//         Test2,
//     }
// }
//
// nightraven! {
//     #[derive(Debug)]
//     enum TopLevel2 {
//         SubVariant,
//         Test1,
//         Test2,
//     }
// }

fn main() {
    // println!("List: {:?}", TopLevel::SubVariant.list_names());
    // println!("Concatenated names: {}", TopLevel::SubVariant.concatenated_names());
}
