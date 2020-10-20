// run-rustfix
#![allow(dead_code)]
#![warn(clippy::unneeded_pub_crate)]

pub(crate) struct Baz {
    pub(crate) field: u8,
}

mod outer {
    mod inner {
        // do these things _really_ need to be `pub(crate)`?
        pub(crate) struct Foo;
        pub(crate) trait Baz {
            fn the_goods(&self);
            fn secret_thing(&self);
        }
        impl Baz for crate::Baz {
            fn the_goods(&self) {}
            fn secret_thing(&self) {}
        }
        pub(crate) fn foo() {}
        pub(crate) fn bar() {
            foo();
            let _ = Foo;
            let x = crate::Baz { field: 3 };
            x.the_goods();
            x.secret_thing();
        }
    }
    pub fn main() {
        inner::bar();
    }
}

fn main() {
    crate::outer::main();
}
