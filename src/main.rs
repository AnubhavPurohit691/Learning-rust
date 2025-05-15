use std::string;

fn main(){
    let stringmy=String::from("hello");
    own(stringmy);
    // own(stringmy.clone());
    println!("{}",stringmy);
}
fn own(stringmy:String){
println!("{}",stringmy);
}