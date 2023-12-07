
pub trait Sport {
   type SportType;

   fn play (&self, st: self::SportType);
}
struct Football;

pub enum SportType{
   Land,
   Water,
}
impl Sport for Football {
    type SportType = SportType;
    fn play (&self, _st: Self::SportType) {
        
    }
    
}
fn main() {

    let f = Football;
    f.play(SportType::Land);
    println!("Hello, world!");
}
