use health_tracker::{query_database, read_file_and_create_database};

fn main() {
    read_file_and_create_database().expect("Couldn't read file");
    query_database().expect("Couldn't query db");
}