use gui::{Screen, Button, SelectBox};

fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelectBox {
                width: 60,
                height: 40,
                option: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
        ],
    };

    //let s = Screen {
    //    components: vec![
    //        Button {
    //            width: 50,
    //            height: 10,
    //            label: String::from("ok"),
    //        },
    //        SelectBox {
    //            width: 60,
    //            height: 40,
    //            option: vec![
    //                String::from("Yes"),
    //                String::from("No"),
    //                String::from("MayBe"),
    //            ],
    //        },
    //    ],
    //};

    s.run();
    println!("Hello, world!");
}
