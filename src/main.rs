
fn main(){
    let mut s1=String::from("anubhav");
    let s2=&s1;//borrowing s1
    println!("{}",s1);
    borrowing(&mut s1);
}
fn borrowing(s: &mut String){
s.push_str("purohit");
}

//you can do 1 mut borrow but if you want to only borrow value u can many