// Trait std::ops::Drop

/*
Custom code within the destructor.

When a value is no longer needed, Rust will run a “destructor” on that value. The most common way that a value is no longer needed is when it goes out of scope.

pub trait Drop {
    // Required method
    fn drop(&mut self);
}
 */

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}

struct HasTwoDrops {
    _one: HasDrop,
    _two: HasDrop,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

#[test]
fn t0() {
    let _x = HasTwoDrops { _one: HasDrop, _two: HasDrop };
    println!("Running!");

    /*
    Even if we remove the implementation of Drop for HasTwoDrop, the destructors of its fields are still called.
    This would result in

    Running!
    Dropping HasDrop!
    Dropping HasDrop!
     */
}

#[test]
fn t1() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");

    // 报错:error[E0040]: explicit use of destructor method
    // c.drop();
    /*
    Rust doesn’t let you call the Drop trait’s drop method manually;
    instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.
     */
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}