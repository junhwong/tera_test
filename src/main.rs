#[macro_use]
extern crate tera;

fn main() {
    use std::env;
    use std::thread;
    use tera::Tera;
    let child = thread::Builder::new().stack_size(2 * 1024 * 1024).spawn(move || {
        let mut ctx = tera::Context::new();
        let tera = compile_templates!("templates/*");
        tera.render("index.html", &ctx);
        println!("PASSED");
    }).unwrap();
    println!("min_stack:{:?}", env::var("RUST_MIN_STACK"));
    println!("max_cached_stacks:{:?}", env::var("RUST_CACHED_STACKS"));
    child.join().unwrap();
}

#[test]
fn test() {
    
    use tera::Tera;
    
    let mut ctx = tera::Context::new();
    //panic!("never returns!");
    let tera = compile_templates!("templates/*");
    tera.render("index.html", &ctx);
}