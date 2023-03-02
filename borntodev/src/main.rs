struct User {
    name: String,
    age: u32,
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let name = "Argo";
    println!("{}", name);
    println!("{}", add(1, 2));
    let argogo = User {
        name: "Argogo".to_string(),
        age: 90,
    };

    println!("{} {}", argogo.name, argogo.age); 
}
