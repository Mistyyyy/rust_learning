pub fn create_new_vec() -> Vec<i32> {
    let val: Vec<i32> = Vec::new();
    return val;
}

pub fn create_initial_vec() -> Vec<i32> {
    vec![1, 2, 3]
}

#[derive(Debug)]
pub enum SpreadSellCell {
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn create_enum_vec() -> Vec<SpreadSellCell> {
    vec![
        SpreadSellCell::Int(20),
        SpreadSellCell::Float(20.1),
        SpreadSellCell::Text(String::from("this is text"))
    ]
}