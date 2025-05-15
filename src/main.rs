// struct are like classes
struct Rec{
    height:u8,
    width:u8
}
impl Rec {
    fn area(&self)->u8{
       return  &self.height*&self.width
    }
}
fn main(){
let rectangle=Rec{
    height:5,
    width:5
};
println!("{}",rectangle.area())
}