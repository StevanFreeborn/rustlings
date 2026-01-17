fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec![];

    for element in vec {
        new_vec.push(*element);
    }

    new_vec.push(88);

    new_vec
}

fn main() {
    println!("Ownership be ownershipping")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
