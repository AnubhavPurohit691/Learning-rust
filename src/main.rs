enum Shape{
    Circle(f32),
    Rectangel(f32,f32)
}
fn calculatearea(shape:Shape)->f32{
match shape{
    Shape::Circle(radius)=>3.14*radius*radius,
    Shape::Rectangel(width,height )=>width*height,
}
}
fn main(){
let circle= Shape::Circle(5.0);
let rectangle=Shape::Rectangel(5.0, 5.0);
println!("{}",calculatearea(circle));
println!("{}",calculatearea(rectangle));
}