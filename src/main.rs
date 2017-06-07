#[macro_use]
extern crate tera;

fn main() {
    use tera::Tera;
    let mut ctx = tera::Context::new();
    //panic!("never returns!");
    let tera = compile_templates!("templates/*");
    tera.render("index.html", &ctx);
}

#[test]
fn test() {
    use tera::Tera;
    let mut ctx = tera::Context::new();
    //panic!("never returns!");
    let tera = compile_templates!("templates/*");
    tera.render("index.html", &ctx);
}