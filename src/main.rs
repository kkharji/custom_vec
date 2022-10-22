use vec::MVec;

fn main() {
    println!("Start");

    let mut vec = MVec::<usize>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec.capacity(), 4);

    assert_eq!(vec.get(0), Some(&1));
    assert_eq!(vec.get(1), Some(&2));

    println!("End");
}
