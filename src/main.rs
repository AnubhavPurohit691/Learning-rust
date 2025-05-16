fn main(){
    // immutable iterator
    let num = vec![2,3,4];
    let iter=num.iter();// it borrowing num value and iterit this function do this
    for i in iter{
        println!("{}",i);
    }
    println!("{:?}",num);


    // mutable iterator

    let mut mutiter=vec![2,3,4,56,6];
    let  mutiteror= mutiter.iter_mut();
    for i in mutiteror{
        *i=*i+1;
    }
    println!("{:?}",mutiter);

    // we can do it using like this using next() 

    // while let Some(val) = mutiteror.next(){
    //     println!("{}",val);
    // }

}