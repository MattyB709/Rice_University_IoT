mod query;
use query::Query;

/* This file contains all algorithms I have currently implemented in Rust.

 All algorithms implement the Query trait, which includes a start and next 
 function to loop through the stream of data to represent a real IoT input stream

 Current algorithms:
 - Find max
 - Average
 - Standard deviation
 - Second greatest
 - Return item from K steps ago
 - Sum over a sliding window
 - Average over a sliding window
 - Max over a sliding window (using a segment tree)
 */
fn main(){
    let stream1 = vec![1,2,3,1,4,3,4,4,3,3,3,2,4,4];
    let stream2 = vec![9,3,11,5,6,121,6,3,3];
    let stream3 = vec![-5,-100,-11,-2, 1, 11, 5];
    let mut second_greatest_struct = SecondGreatest::new();
    second_greatest_struct.start();
    for i in 0..stream1.len(){
        println!("{}", second_greatest_struct.next(stream1[i]));
    }
}

struct FindMax{
    max: Option<i64>,
}

impl FindMax{
    fn new() -> FindMax{
        FindMax{max: None}
    }
}

impl Query<i64,i64> for FindMax{
    fn start(&mut self){
        self.max = None;
    }
    fn next(&mut self, item:i64) -> i64 {
        match self.max{
            None => self.max = Some(item),
            Some(x) => if item > x{
                self.max = Some(item);
            }
        }
        self.max.unwrap()
    }
}

struct Average{
    sum: f64,
    index: f64
}

impl Average{
    fn new() -> Average{
        Average{sum: 0.0, index:0.0}
    }
}

impl Query<i64,f64> for Average{
    fn start(&mut self){
        self.sum = 0.0;
        self.index = 0.0;
    }
    fn next(&mut self, item:i64) -> f64 {
        self.sum += item as f64;
        self.index +=1.0;
        self.sum/self.index
    }
}

struct StDev{
    sum: f64,
    sumsq: f64,
    index: f64,
    mean: f64
}

impl StDev{
    fn new() -> StDev{
        StDev{sum: 0.0, sumsq:0.0, index:0.0, mean:0.0}
    }
}

impl Query<i64,f64> for StDev{
    fn start(&mut self){
        self.sum = 0.0;
        self.sumsq = 0.0;
    }
    fn next(&mut self, item:i64) -> f64 {
        self.sum += item as f64;
        self.sumsq += item as f64 * item as f64;
        self.index += 1.0;
        self.mean = self.sum/ self.index;
        (self.sumsq/self.index-(self.mean* self.mean)).sqrt()
    }
}

struct MostFrequent{
    arr: [u8; 256],
    frequency: u8,
    most_frequent: u8,
}

impl MostFrequent{
    fn new() -> MostFrequent{
        MostFrequent{arr:[0;256], frequency:0, most_frequent:0}
    }
}

impl Query<u8, u8> for MostFrequent{
    fn start(&mut self){
        self.arr = [0;256];
        self.frequency = 0;
        self.most_frequent = 0;
    }
    fn next(&mut self, item:u8) -> u8{
        self.arr[item as usize] += 1;
        if self.arr[item as usize]  > self.frequency{
            self.frequency = self.arr[item as usize];
            self.most_frequent = item;
        }
        else if self.arr[item as usize] == self.frequency && item > self.most_frequent{
            self.most_frequent = item;
        }
        self.most_frequent
    }
}

struct SecondGreatest{
    greatest:i64,
    second_greatest:i64,

}

impl SecondGreatest{
    fn new() -> SecondGreatest{
        SecondGreatest{greatest: std::i64::MIN, second_greatest: std::i64::MIN}
    }
}

impl Query<i64, i64> for SecondGreatest{
    fn start(&mut self){
        self.greatest = std::i64::MIN; 
        self.second_greatest = std::i64::MIN;
    }
    fn next(&mut self, item:i64) -> i64{
        if item > self.greatest{
            self.second_greatest = self.greatest;
            self.greatest = item;
            
        }
        else if item > self.second_greatest && item < self.greatest{
            self.second_greatest = item;
        }
        self.second_greatest
    }
}

struct KStepsAgo{
    vector:Vec<Option<i64>>,
    index:usize,
    window_length:usize
}

impl KStepsAgo{
    fn new(k: usize) -> KStepsAgo{
        KStepsAgo{vector: vec![None;k], index:0, window_length:k}
    }
}

impl Query<i64, ()> for KStepsAgo{
    fn start(&mut self){
        self.vector = vec![None;self.window_length];
        self.index = 0;
    }
    fn next(&mut self, item:i64){
        match self.vector[self.index]{
            None=>println!("nothing found"),
            Some(x)=>println!("{}", x)
        }
        self.vector[self.index] = Some(item);
        
    }
}

// Sliding Window Sum implementation

struct SlidingWindowSum{
    window_length:usize,
    sum:i64,
    index: usize,
    sliding_window: Vec<i64>
}

impl SlidingWindowSum{
    fn new(k: usize)->SlidingWindowSum{
        SlidingWindowSum {window_length: k, sum: 0, index: 0, sliding_window: vec![0;k]}
    }
}

impl Query<i64, i64> for SlidingWindowSum{

    fn start(&mut self){
        for i in 0..self.window_length-1{
            self.sliding_window[i] = 0;
        }
        self.sum = 0;
        self.index = 0;
    }

    fn next(&mut self, item: i64) -> i64{
        // Reset the index back to zero when it hits the edge of the window
        if self.index == self.window_length{
            self.index = 0;
        }
        // subtract what's currently in the index (0 at initialization) and then replace and add the current item to find new sum
        self.sum -= self.sliding_window[self.index];
        self.sliding_window[self.index] = item;
        self.sum += self.sliding_window[self.index];
        self.index+=1;
        self.sum

    }
}

// Finds the average of all values within a sliding window

struct SlidingWindowAverage{
    window_length:usize,
    sum:i64,
    index: usize,
    sliding_window: Vec<i64>,
    full_window:bool,
    
}

impl SlidingWindowAverage{
    fn new(k: usize)->SlidingWindowAverage{
        SlidingWindowAverage {window_length: k, sum: 0, index: 0, sliding_window: vec![0;k], full_window: false}
    }

}

impl Query<i64, f64> for SlidingWindowAverage{

    fn start(&mut self){
        // Reset all values in the window to zero
        for i in 0..self.window_length-1{
            self.sliding_window[i] = 0;
        }
        self.sum = 0;
        self.index = 0; 
        self.full_window = false;
    }

    fn next(&mut self, item: i64) -> f64{
        // Reset the index back to zero when it hits the edge of the window
        if self.index == self.window_length{
            self.index = 0;
            self.full_window = true;
        }
        // subtract what's currently in the index (0 at initialization) and then replace and add the current item to find new sum
        self.sum -= self.sliding_window[self.index];
        self.sliding_window[self.index] = item;
        self.sum += self.sliding_window[self.index];
        // update the index, meaning you are now looking at the next point in the window
        self.index+=1;
        // On iterations before the full window is filled, the sum is divided by the index instead
        let divisor = 
        if !self.full_window{ self.index } 
        else { self.window_length };

        self.sum as f64 / divisor as f64

    }
}

// Sliding Window Max (less efficient version)

struct SlidingWindowMax{
    max:i64,
    window_length:usize,
    index:usize,
    tree:Vec<i64>
}

// Created sliding window functions

impl SlidingWindowMax{
    fn new(k: usize)->SlidingWindowMax{
        SlidingWindowMax{max: 0,window_length: k, index:0, tree:vec![0;k*4]}
    }

    fn find_max(&mut self, first: i64, second: i64) ->i64{
        match first > second{
            true=>first,
            false=>second
        }
    }

    // Creating the tree functions

    fn build_tree(&mut self, left_border: usize, right_border: usize, vertex: usize){
        if left_border == right_border{
            self.tree[vertex] = std::i64::MIN;
        }
        else{
            let middle = (left_border + right_border) / 2;
            self.build_tree(left_border, middle, vertex * 2);
            self.build_tree(middle + 1, right_border, vertex*2 + 1);
            self.tree[vertex] = std::i64::MIN;
        }
        
    }

    // updating the max for the tree

    fn tree_max(&mut self, value: i64, vertex: usize,index: usize, left_border: usize, right_border: usize){
        // base case, initialize the bottom value to the value in the window
        if left_border == right_border{
            self.tree[vertex] = value;
        }
        else{
            // Goes "down" the tree
            let middle = (left_border + right_border)/2;
            if index <= middle{
                self.tree_max(value, vertex*2, index, left_border, middle);                
            }
            else if index > middle{
                self.tree_max(value, vertex*2+1, index, middle + 1, right_border);
            } 
            self.tree[vertex] = self.find_max(self.tree[vertex * 2], self.tree[vertex * 2 + 1])
        }
    }



}


impl Query<i64, i64> for SlidingWindowMax{
    fn start(&mut self) {
        self.max = 0;
        self.index=0;
        self.build_tree(0, self.window_length-1, 1)
    }

    fn next(&mut self, item: i64) -> i64 {
        if self.index == self.window_length{
            self.index = 0;
        }
        self.tree_max(item, 1, self.index, 0, self.window_length-1);
        self.index+=1;

        // Simple tree visualizer
        let mut k = 1;
        for i in 0..4
        {
            for _j in 0..i32::pow(2, i)
            {
                if self.tree[k] == std::i64::MIN{
                    print!("min ");
                }
                else{
                    print!("{} ", self.tree[k]);
                }
                k+=1;
            }
            println!();
        }
        self.tree[1]
    }
}



