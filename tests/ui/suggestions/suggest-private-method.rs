mod structs {
    pub struct Sun {
        // this is the field we want but can't get as it's not pub
        earth: usize,
        pub next: RedGiant,
    }

    impl Sun {
        fn earth(&self) -> usize {
            self.earth

        }
    }

    pub struct RedGiant {
        // this field is unfortunately named the same but is pub
        earth: bool,
    }

    impl RedGiant {
        pub fn earth(&self) -> bool {
            self.earth

        }
    }

    impl std::ops::Deref for Sun {
        type Target = RedGiant;

        fn deref(&self) -> &Self::Target {
            &self.next
        }
    }
}

use structs::Sun;
fn assign_method_result(star: Sun) {
    let res: usize = star.earth(); //~ ERROR mismatched types
}

fn main() {}

// rename private method deref
// add enum as seperate test
