use std::ops::Deref;

struct MyBox<T>(T);

#[allow(dead_code)]
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[allow(dead_code)]
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_mybox_ref() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn use_implicit_deref() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        assert_eq!("Rust", *m);
    }

    #[test]
    fn use_explicit_deref() {
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);
        assert_eq!("Rust", *m);
    }

    #[test]
    fn example_des_ref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn example_box_ref() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
