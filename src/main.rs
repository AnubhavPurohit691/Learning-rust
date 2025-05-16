use std::collections::HashMap;

fn main(){
let complexhashmap=vec![(String::from("anubhav"),4),(String::from("purohit"),5)];
let getval= val(complexhashmap);
println!("{:?}",getval);
}


fn val(v:Vec<(String,u32)> )->HashMap<String,u32>{
let mut values=HashMap::new();
for (key,value) in v{
    values.insert(key,value);
};
return values;
}