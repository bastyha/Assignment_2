fn main(){

    let mut numbers: Vec<String> =vec![String::from("1");1000];
    
    for i in 1..1001{
        for _j in 1..(i+1){
            let mut local_number:u64 = numbers[(i-1)].parse().unwrap();
            local_number = local_number * i.to_string().parse::<u64>().unwrap();

            let length = local_number.to_string().len();
            if length >10{
                numbers[(i-1)] = (local_number.to_string()[length-10..]).to_string();
            }
            else{
                numbers[(i-1)] = local_number.to_string();
            }
        }
    } 
    println!("{:?}", numbers);

    let mut sum:String =String::from("0"); 
    
    for number in numbers{
        let local_sum:u64 = sum.parse::<u64>().unwrap() + number.parse::<u64>().unwrap();
        let length = local_sum.to_string().len();
        if  length >10{
            sum = (local_sum.to_string()[length-10..]).to_string();
        }
        else{
            sum = local_sum.to_string();
        }
    }
    println!("the last 10 number of the sum: {sum}");
}