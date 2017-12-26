extern crate opencv;

use std::thread;

use self::opencv::core::{Mat, absdiff, bitwise_and, Size, Point, Scalar};
use self::opencv::highgui::{VideoCapture, imshow, wait_key};
use self::opencv::imgproc::{threshold, CV_THRESH_BINARY, cvt_color, CV_RGB2GRAY, MORPH_RECT, get_structuring_element, erode};

pub struct CameraReader;


impl CameraReader {
    pub fn new() -> Self {
        return CameraReader
    }

    pub fn loop_reading(&self, index: i32) -> thread::JoinHandle<Result<String,String>> {
        let thread_camera_reader: thread::JoinHandle<Result<String,String>> = thread::spawn(move || {
            let mut cam = try!(VideoCapture::device(index));
            let opened = try!(VideoCapture::is_opened(&cam));

            let mut frame = try!(Mat::new());
            let mut prev_frame = try!(Mat::new());
            let mut current_frame = try!(Mat::new());

            let mut d1 = try!(Mat::new());
            let mut d2 = try!(Mat::new());
            let mut motion = try!(Mat::new());
            let mut mask = try!(Mat::new());
            let mut pMOG2 = opencv::video::BackgroundSubtractorMOG2::default().unwrap();

            if !opened {
                return Err(String::from("Deu bosta"))
            }

            loop {
                let mut kernel_ero = get_structuring_element(MORPH_RECT, Size{ width: 2, height: 2}, Point{x: 0, y: 0});

                prev_frame = current_frame.clone().unwrap();
                current_frame = frame.clone().unwrap();

                try!(cam.read(&mut frame));

                cvt_color(&frame, &frame, CV_RGB2GRAY, 0);

                // pMOG2.apply();

                absdiff(&prev_frame, &frame, &d1);
                absdiff(&frame, &current_frame, &d2);

                bitwise_and(&d1, &d2, &motion, &mask);
                threshold(&motion, &motion, 35.0, 255.0, CV_THRESH_BINARY);
                erode(&motion, &motion, &kernel_ero.unwrap(), Point{x: 0, y: 0}, 0, 0, Scalar{data: [0.0, 0.0, 0.0, 0.0]});

                if try!(motion.size()).width > 0 {
                    try!(imshow("asd", &mut motion));
                }
                if try!(wait_key(10)) == 27 {
                    break;
                }
            }

            Ok(String::from("Ok"))
        });

        thread_camera_reader
    }
}
