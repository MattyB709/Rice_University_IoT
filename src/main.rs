mod query;
use query::Query;

// My most recent algorithm (sliding window max)

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
    fn update_max(&mut self, vertex: usize){
        // Base case
        if vertex == 1{
            self.tree[vertex] = self.find_max(self.tree[vertex*2], self.tree[vertex*2+1]);
        }
        else{
            // Before it hits the root, move upward through the tree and update at each point
            self.tree[vertex] = self.find_max(self.tree[vertex*2], self.tree[vertex*2+1]);
            self.update_max(vertex/2);
            
        }
    }
}

// Implementing Query Trait

impl Query<i64, i64> for SlidingWindowMax{
    // On start, initialize an empty tree with values of MIN
    fn start(&mut self) {
        self.max = 0;
        self.index=0;
        self.build_tree(0, self.window_length-1, 1)
    }

    fn next(&mut self, item: i64) -> i64 {
        if self.index == self.window_length{
            self.index = 0;
        }
        self.tree[self.window_length + self.index] = item;
        self.update_max((self.window_length + self.index)/2);
        self.index+=1;
        self.tree[1]
    }
}

fn main(){

    let stream1 = vec![1,2,3,1,7,3,4,4,3,3,3,2,4,4, 11, 3, 2, 3, 5 , 4 ,3, 2];
    let _stream2 = vec![9,3,11,5,6,121,6,3,3];
    let _stream3 = vec![-5,-100,-11,-2, 1, 11, 5, -3, -4, -10];
    let mut sliding_window_struct = SlidingWindowMax::new(3);
    
    sliding_window_struct.start();
    for i in 0..stream1.len(){
        println!("input: {}", stream1[i]);
        println!("Output: {}", sliding_window_struct.next(stream1[i]));
        println!();
    }
}