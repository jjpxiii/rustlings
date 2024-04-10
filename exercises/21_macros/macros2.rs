// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    a::my_macro!();
}

pub mod a {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub(crate) use my_macro;
}
