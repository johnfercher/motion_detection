extern crate opencv;

mod camera_finder;
mod camera_reader;

use camera_reader::CameraReader;

fn main() {
    let camera_reader = CameraReader::new();
    let thread_camera_reader = camera_reader.loop_reading(0);
    thread_camera_reader.join();
}
