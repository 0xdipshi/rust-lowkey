
//result ->error handling 

use std::fs;
fn main(){
    
    let file_content = fs::read_to_string("file.txt");  
    //options 
    match file_content{
        
        Ok(value)=>{
            println!("{}",value);
        },
        Err(value)=>{
            println!("{}",value);
        }   
    }

    // let  mut _varname =String::from("");
    // println!("{}",get_len(varname.clone()));
  
}

// fn get_len(s:String)->usize{
//   s.len()
// }
