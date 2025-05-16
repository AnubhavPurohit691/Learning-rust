fn main (){
    let mut vec= Vec::new();
    let number = vec![1,2,4];//another way
    println!("{:?}",number);
    vec.push(1);
    vec.push(2);
    println!("{:?}",vec);//vec are structs.
    let nvec=new_vec(vec);
    println!("{:?}",nvec);
    // println!("{:?}",vec);//vec are structs.
}
fn new_vec(vec:Vec<i32>)->Vec<i32>{
    let mut nvec=Vec::new();
    for val in vec{
        if val%2==0{
            nvec.push(val);
        }
    }
return nvec
}