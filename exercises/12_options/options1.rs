// This function returns how much ice cream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no ice cream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        return None;
    }

    if hour_of_day >= 22 {
        return Some(0);
    }

    Some(5)
}

fn main() {
    println!("This was took way to long")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams.unwrap(), 5);
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
