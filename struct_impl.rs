

//struct and impl 

struct Rect {
    width: u32,
    height: u32,

}

impl Rect{

    fn area(&self) -> u32{
        self.width * self.height
    }

    fn perimeter(&self) -> u32{
        2*(self.width * self.height)
    }

    fn print_smtg(){
        print!("this is a static cannot access the members also cannot be called using obj . ");
    }
}

fn main(){

    let r = Rect{
        height: 120,
        width: 120,

    };

    println!("{}", r.area());
    println!("{}",r.perimeter());

    Rect::print_smtg();      //static implemented so call on the impl 

    println!("the height and width is {:?} {:?}",r.height, r.width);


}
