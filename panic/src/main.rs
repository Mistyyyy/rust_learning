mod panic;
mod result;

fn main() {
    // panic::index_out_of_bound();

    result::try_open_file();

    result::try_another_open();

    // result::try_unwrap_expect();

    // result::read_username_from_file().expect("hi");

    result::read_username_from_file_with_from_trait().expect("aaa");
}
