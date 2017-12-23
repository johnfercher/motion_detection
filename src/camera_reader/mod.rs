extern crate opencv;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::io;
use std::fs::File;
use std::marker;
use std::io::prelude::*;

use self::opencv::core;
use self::opencv::highgui;

pub struct CameraReader;

impl CameraReader {
    pub fn new() -> Self {
        return CameraReader
    }

    pub fn loop_reading(index: i32, sender: Sender<core::Mat>) -> /*thread::JoinHandle<Result<String,String>>*/ () {
        let thread_camera_reader: thread::JoinHandle<Result<String,String>> = thread::spawn(move || {
            let mut cam = try!(highgui::VideoCapture::device(index));
            let opened = try!(highgui::VideoCapture::is_opened(&cam));
            let mut frame = try!(core::Mat::new());

            if !opened {
                return Err(String::from("Deu bosta"))
            }

            loop {
                try!(cam.read(&mut frame));
                sender.send(frame);
            }

            Ok(String::from("Ok"))
        });

        //thread_camera_reader
    }
}
