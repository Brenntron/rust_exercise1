pub fn main() {
    let vec = fill_vec(Vec::new());

    println!("vec has {:?} (length is {})", vec, vec.len());
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    return vec.clone();
}
