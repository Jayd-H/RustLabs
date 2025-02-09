mod shared_data;
use shared_data::SharedData;

fn main() {

    let shared_data = SharedData::new();
    shared_data.print();
}

fn update_print(shared_data: &SharedData) {
    shared_data.update();
    shared_data.print();
}
