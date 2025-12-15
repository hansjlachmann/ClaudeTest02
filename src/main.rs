use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Hello, {}!", args[1]);
    } else {
        println!("Hello, World!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_string_formatting() {
        let name = "Rust";
        let greeting = format!("Hello, {}!", name);
        assert_eq!(greeting, "Hello, Rust!");
    }
}
