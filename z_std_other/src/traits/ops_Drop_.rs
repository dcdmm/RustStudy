// Trait std::ops::Drop

/*
Custom code within the destructor.

When a value is no longer needed, Rust will run a “destructor” on that value. The most common way that a value is no longer needed is when it goes out of scope.

You cannot implement both Copy and Drop on the same type. Types that are Copy get implicitly duplicated by the compiler, making it very hard to predict when, and how often destructors will be executed. As such, these types cannot have destructors.
 */

// # Implementors
/*
unsafe impl<#[may_dangle] T, A: Allocator> Drop for Vec<T, A> 

unsafe impl<#[may_dangle] T, A: Allocator> Drop for VecDeque<T, A>

unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for Box<T, A>

unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for Rc<T, A>

unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for Weak<T, A>
*/

#[allow(warnings)]
#[test]
fn t0() {
    struct HasDrop {
        data: String,
    }
    
    impl Drop for HasDrop {
        fn drop(&mut self) {
            println!("Dropping HasDrop {}!", self.data);
        }
    }
    
    struct HasTwoDrops {
        one: HasDrop,
        two: HasDrop,
    }
    
    impl Drop for HasTwoDrops {
        fn drop(&mut self) {
            println!("Dropping HasTwoDrops!");
        }
    }

    let _x = HasTwoDrops {
        one: HasDrop {
            data: String::from("1"),
        },
        two: HasDrop {
            data: String::from("2"),
        },
    };
    /*
    print->
    Dropping HasTwoDrops!
    Dropping HasDrop 1!
    Dropping HasDrop 2!
     */
    println!("Running!");

    /*
    Even if we remove the implementation of Drop for HasTwoDrop, the destructors of its fields are still called.
    This would result in

    Running!
    Dropping HasDrop 1!
    Dropping HasDrop 2!
     */
}

