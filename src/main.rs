fn main() {
    println!("Hello, world!");
    let ans = sum(5,3);
    println!("{}",ans); //this is how you get variable in rust .
    println!("{}",is_even(ans));
    stringtest();
    vec();

    // for loop

    for i in 0..100{
        println!("{}",i);
    }
}
// integer
fn sum(a:i32,b:i32)->i32{
return a + b;
}
// boolean

fn is_even(a:i32)->bool{
return a%2 ==0;
}

// strings

fn stringtest(){
    let name=String::from("anubhav");
    println!("fristname - {}",name);
}
// vector

fn vec(){
    let v = vec![1,2,3];
    println!("{:?}",v);
    }
