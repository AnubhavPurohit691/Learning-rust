pub trait Summary {
    fn summarize(&self)->String;
}
struct User{
    email:String,
    name:String
}
impl Summary for User {
    fn summarize(&self)->String {
        return format!("user email is {} name is {}",self.email,self.name); // format concatinate  the string
    }
}
fn main(){
let user = User{
    email:String::from("anubhav"),
    name:String::from("anubhavpurohit")
};
println!("{}",user.summarize());
notify(user);
}
//impl trait
//trait as parameter

fn notify(u:impl Summary){//this is syntac sugar for below code
println!("{}",u.summarize());
}

// fn notify<T:Summary>(items:T){
//     println!("{}",items.summarize());
// }