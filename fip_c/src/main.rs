//! TODO: documentation

fn hello() {
    extern "C" {
        fn hello();
    }

    #[allow(unsafe_code)]
    unsafe {
        hello();
    }
}

fn main() {
    hello();
}
