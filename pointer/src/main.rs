fn main() {
    let mut vec1 = vec![1, 2, 3];

    println!("The Vecs is {:?}", vec1[0]);

    {
        let var1 = &mut vec1[0];

        *var1 = 3;
    }

    println!("The Vecs is {:?}", vec1[0]);
}
