fn main() {
    let mut arr = vec![1, 2, 3];
    // cache the last item
    let temp = arr.clone();
    let last = temp.last();
    arr.push(4);
    // consume previously stored last item
    println!("last: {:?}", last);
}
