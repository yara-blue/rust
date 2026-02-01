mod structs {
    pub struct A {
        // this is the field we want but can't get as it's not pub
        field: usize,
        b: B,
    }

    impl A {
        fn method(&self) -> usize {
            self.field
        }
    }

    pub struct B {
        // this field is unfortunately named the same but is pub
        pub field: bool,
    }

    impl B {
        pub fn method(&self) -> bool {
            self.field
        }
    }

    impl std::ops::Deref for A {
        type Target = B;

        fn deref(&self) -> &Self::Target {
            &self.b
        }
    }
}

use structs::A;
fn try_to_use_field_in_a(a: A) {
    a.field + 5; //~ ERROR cannot add `{integer}` to `bool` [E0369]
    // help should mention we are de-reffing because A::field is not public
}

fn assign_field(a: A) {
    let res: usize = a.field; //~ ERROR mismatched types
}

fn try_to_use_method_in_a(a: A) {
    a.method() + 5; //~ ERROR cannot add `{integer}` to `bool` [E0369]
    // help should mention we are de-reffing because A::field is not public
}

fn assign_method(a: A) {
    let res: usize = a.method(); //~ ERROR mismatched types
}

fn main() {}
