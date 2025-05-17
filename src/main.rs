//lifetime in structs

use std::fmt::Display;


// struct User<'a>{
//     name:&'a str
// }
// fn main(){
// let firstname=String::from("anubhav");
// let user=User{
//     name:&firstname
// };
// println!("{}",user.name);
// }
fn main(){
let xname=String::from("anubhav");
let yname=String::from("purohit4");
let ans =longest_with_an_announcement(&xname, &yname, "Comparing names");
println!("{}",ans)
}
fn longest_with_an_announcement<'a,T>(x:&'a str,y:&'a str,ann:T)->&'a str where T:Display,{
println!("Announcement! {ann}");
if x.len()>y.len(){
    return x
}
else{
    return y
}
}

