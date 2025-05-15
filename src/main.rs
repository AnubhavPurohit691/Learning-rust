
fn main(){
    let mut stringmy=String::from("hello");
     stringmy=own(stringmy);
    // own(stringmy.clone());
    // println!("{}",stringmy);
    println!("{}",stringmy);

}
fn own(stringmy:String)->String{
println!("{}",stringmy);
return stringmy
}