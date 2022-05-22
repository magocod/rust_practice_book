pub mod jwt;
pub mod mongo;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn optional_add(x: i32, y: Option<i32>) -> i32 {
    // x + y.or_else(|| { Some(1) })
    match y {
        Some(v) => x + v,
        None => x + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }

    #[test]
    fn not_works() {
        assert_ne!(add_one(3), 5);
    }

    #[test]
    fn default_optional_add() {
        assert_eq!(optional_add(1, None), 2);
    }

    #[test]
    fn send_optional_add() {
        assert_eq!(optional_add(3, Some(2)), 5);
    }

    #[test]
    fn vector() {
        let c = 10;
        let mut v: Vec<usize> = Vec::with_capacity(c);

        for i in 0..c {
            v.push(i);
        }

        // println!("{:?}", v);
        assert_eq!(v.len(), 10)
    }

    #[test]
    fn option() {
        let c = 10;
        let mut v: Vec<usize> = Vec::with_capacity(c);

        for i in 0..c {
            v.push(i);
        }

        // println!("{:?}", v);
        assert_eq!(v.len(), 10)
    }
}
