fn picky_eater(food: &str) -> &str {
    match food {
        "strawberry" => "Yummy!",
        "potato" => "I guess I can eat that.",
        "broccoli" | "gummy bears" => "No thanks!",
        _ => "No thanks!",
    }
}

fn main() {
    println!("{}", picky_eater("strawberry"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
