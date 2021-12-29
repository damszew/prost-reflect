use prost_reflect_derive::ReflectMessage;

#[derive(ReflectMessage)]
#[prost_reflect(
    file_descriptor_set_path = "file_descriptor_set.bin",
    message_name = "msg"
)]
#[prost_reflect(
    file_descriptor_set_path = "file_descriptor_set.bin",
    message_name = "msg"
)]
pub struct MyMessage {}

fn main() {}
