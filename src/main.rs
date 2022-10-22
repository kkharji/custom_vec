use vec::MVec;

#[derive(Debug, PartialEq)]
struct Num(usize);

impl Drop for Num {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    println!("Start");

    let mut vec = MVec::<Num>::new();
    vec.push(Num(1));
    vec.push(Num(2));
    vec.push(Num(3));
    vec.push(Num(4));

    assert_eq!(vec.len(), 4);
    assert_eq!(vec.capacity(), 4);

    {
        let (first, second) = (Num(1), Num(2));
        assert_eq!(vec.get(0), Some(&first));
        assert_eq!(vec.get(1), Some(&second));
    }

    println!("End");
}
