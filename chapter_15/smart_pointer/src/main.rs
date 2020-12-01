struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[allow(unused_variables)]
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let f = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer f created.");
    drop(f);
    println!("CustomSmartPointer dropped before the end of main.");
}
