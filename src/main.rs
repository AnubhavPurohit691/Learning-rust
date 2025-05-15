struct User{
    email:String,
    password:u32
}
fn main(){
    
let user1=User{
    email:String::from("anubhav"),
    password:5,
};
println!("{}",user1.email)
// tuple struct, unit struct have to learn it
}