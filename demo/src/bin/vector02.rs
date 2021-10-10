struct Test {
    score: i32,
    name: &'static str,
}

fn main() {
    let my_scores = vec![
        Test { score: 90, name: "CSS" },
        Test { score: 80, name: "Node JS" },
        Test { score: 77,  name: "Three JS" },
        Test { score: 93, name: "JavaScript" },
    ];

    for test in my_scores {
        println!("{:?}'s score = {:?}", test.name, test.score);
    }
}
