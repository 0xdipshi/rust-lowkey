
enum Shapes{
    
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64)
}

fn main(){
    
    let _circle = Shapes::Circle(10.0);
    let square = Shapes::Square(4.0);
    let rect = Shapes::Rectangle(3.0,2.0);
    
    println!("Area of rect ={0} and area of square ={1}",get_area(rect),get_area(square));

}


fn get_area(shape:Shapes)->f64{
    
    //pattern matching 
    let value =match shape{
        Shapes::Rectangle(a,b)=>a*b,
        Shapes::Circle(r)=>3.14*r*r,
        Shapes::Square(a)=>a*a
    };
    
    value
    
}
