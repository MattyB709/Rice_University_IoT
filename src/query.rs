pub trait Query<A,B>{
    fn start(&mut self);
    fn next(&mut self, item: A) -> B;
}