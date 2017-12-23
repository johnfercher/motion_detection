extern crate opencv;

mod camera_finder;
mod camera_reader;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use camera_finder::CameraFinder;
use camera_reader::CameraReader;
use opencv::core;

fn main() {
    let mut camera_finder = CameraFinder::new();
    camera_finder.find_cameras().unwrap();

    let (sender, receiver): (Sender<core::Mat>, Receiver<core::Mat>) = mpsc::channel();
    let mut camera_reader = CameraReader::new();
    let thread_camera_reader = camera_reader.loop_reading(camera_finder.first_or_default_camera(), sender);
}
