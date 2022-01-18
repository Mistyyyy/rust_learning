pub fn use_panic() {
    panic!("crash and burn");
}

pub fn index_out_of_bound() {
    let vec = vec![1, 2, 3];

    vec[99];
}