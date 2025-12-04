

fn main(){

    let nums:[i32;10] =[1,2,3,4,5,6,7,8,9,10];

    let value = binary_search(nums);    //nums.clone() or using reference also we can achieve this

    if value == usize::MAX {
        print!("Not found!");
    }
    println!("the target is presented in {}",value);
}


fn binary_search(number:[i32;10])->usize{

    //has to be mutable
    let mut low =0;
    let mut high =number.len()-1;

    let target = 8;


    while low<=high{

        let mid = low+((high-low)/2);
        if number[mid] == target{
            return mid;
        }else if number[mid]>target {
            high=mid-1;
        }else{
            low=mid+1;
        }
    }

    //for false case 
    return usize::MAX;    
}
