use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::<usize>::new_with_capacity(5);
    // dbg!(&v);
    // println!("Length: {}", v.len());
    // println!("Capacity: {}", v.capacity());
    v.push(10);
    let e = v.get(0);
    println!(
        "e = {}, v's length = {}, cap = {}",
        e.unwrap(),
        v.len(),
        v.capacity()
    );
    let e = v.pop();
    println!(
        "e = {}, v's length = {}, cap = {}",
        e.unwrap(),
        v.len(),
        v.capacity()
    );
    for i in 0..6 {
        v.push(i)
    }
    println!(
        "e = {}, v's length = {}, cap = {}",
        e.unwrap(),
        v.len(),
        v.capacity()
    );
}
