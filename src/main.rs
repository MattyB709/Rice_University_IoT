//use std::vec;
//use rand::Rng;

fn main() {
    let stream1 = vec![1,2,3,1,4,3,4,4,3,3,3,2,4,4];
    let stream2 = vec![9,121,3,11,5,6,6,3,3];
    let stream3 = vec![5,100,11,2];
    println!("{}", most_frequent(&stream1));
    println!("{}", most_frequent(&stream2));
    println!("{}", most_frequent(&stream3));
}

fn find_max(x: &Vec<i16>)->i16{
    let mut greatest: i16 = x[0];
    for i in 1..x.len(){
        if x[i] > greatest{
            greatest = x[i];
        }
    }
    greatest
}

fn running_average (x: &Vec<i16>)->f32{
    let mut sum = 0;
    for num in x{
        sum += num;
    }
    (sum as f32)/(x.len() as f32)
}

fn stdev (x: &Vec<i16>)->f32{
    let mut sum = 0.0;
    let mut sumsq = 0.0;
    let n = x.len() as f32;
    for i in 0..x.len(){
        let x = x[i] as f32;
        sum += x;
        sumsq += x*x;
    }
    let mean = sum/n;
(sumsq/n-(mean.powf(2.0))).sqrt()
}

fn most_frequent(x: &Vec<u8>)->u8{
    let mut arr: [u8; 256] = [0; 256];
    let mut frequency: u8 = 0;
    let mut most_frequent: u8 = 0;
    for &num in x{
        arr[num as usize]+=1;
        if arr[num as usize]  > frequency{
            frequency = arr[num as usize];
            most_frequent = num;
        }
        else if arr[num as usize] == frequency && num > most_frequent{
            most_frequent = num as u8;
        }
    }
    most_frequent
}

fn second_greatest(x: &Vec<i16>)->i16{
    let mut greatest = x[0];
    let mut second: i16 = 0;
    for i in 1..x.len(){
        if x[i] > greatest{
            second = greatest;
            greatest = x[i];
        }
        else if x[i] > second && x[i] < greatest{
            second = x[i];
        }
    }
    second
}

/*fn ksteps(x: Vec<i16>, k: i16){
    for i in k..x.len(){
        println!(x[i-k]);
    }
}*/