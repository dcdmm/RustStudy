// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("fly ===> Pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("fly ===> Wizard");
    }
}

impl Human {
    fn fly(&self) {
        println!("fly ===> Human");
    }
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Dog")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Animal")
    }
}

#[test]
fn t0() {
    let person = Human;
    person.fly(); // fly ===> Human
    Pilot::fly(&person); // fly ===> Pilot
    Wizard::fly(&person); // fly ===> Wizard

    /*
    However, associated functions that are not methods don’t have a self parameter.
    When there are multiple types or traits that define non-method functions with the same function name, Rust doesn't always know which type you mean unless you use fully qualified syntax.
     */
    println!("A baby dog is called a {}", Dog::baby_name());

    // Using fully qualified syntax to specify that we want to call the baby_name function from the Animal trait as implemented on Dog
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}