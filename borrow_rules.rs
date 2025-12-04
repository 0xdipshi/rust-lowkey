

// ownership and borrow rules

fn main(){

    // if one mutable reference has been made then no other imm or mut can be created

    //1
    // multiple immutable references
    let name =String::from("rover");
    let _new_name =&name;
     let _new_name1 = &name;


    //2
    //one mutable life ends and multiple immutable references
    let mut example = String::from("Honey singh");
    let new_example   =&mut example ;
    new_example.clear();            //life ends here any opertaion 

    // imm references 
    let new_example1 =&example;
    let _new_example2 =&example;


    print!("{} {}",example,new_example1);


}
