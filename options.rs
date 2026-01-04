
//Options
fn main(){
    
    let index = get_index_of_first_a(String::from("dharshan")); 
    match index{
        
        Some(value)=>println!("the index is {}",value),
        None =>println!("Not found!"),
    };
}

fn get_index_of_first_a(s:String)->Option<i32>{
    for(index,value) in s.chars().enumerate(){
        if value=='a'{
            return Some(index as i32);
        }
    }
    return None;
}
