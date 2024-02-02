fn main() {
    codegen::run();
}

#[cfg(feature = "codegen")]
mod codegen;

#[cfg(not(feature = "codegen"))]
mod codegen {
    pub fn run() {
        println!("Skipping codegen");
    }
}
