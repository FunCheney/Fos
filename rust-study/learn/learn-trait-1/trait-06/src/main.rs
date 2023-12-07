trait Shape {
    fn area(&self) -> f64;
    fn play(&self){
        println!("1");
    }
}

trait Circle : Shape {
    fn radius(&self)->f64{
        2.0
    }
    

    fn play(&self){
        println!("2");
    }
}
struct A;

impl Shape for A{
   fn area(&self) -> f64 {
       3.0
   } 
}

impl Circle for A {
    
}

impl A {
    fn play(&self) {
       println!("3"); 
    }
    
    
}

fn main() {
    let a = A;
    a.play();
    <A as Circle>::play(&a);
    <A as Shape>::play(&a);
    println!("Hello, world!");
}
