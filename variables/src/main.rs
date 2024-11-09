fn main() {
    fun(5, 7);

    let x = String::from("Iam a string");

    let c = test(&x);

    println!("{c}, {x}")
}

fn fun(x: i32, y: i32) -> i32 {
    x + y
}

fn test(x: &String) -> String {
    for (i, c) in x.chars().enumerate() {
        if c == ' ' {
            return x[..i + 1].to_string();
        }
    }

    x[..].to_string()
}
