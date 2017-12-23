extern crate opencv;

use self::opencv::highgui;

pub struct CameraFinder {
    all_cameras: Vec<i32>
}

impl CameraFinder {
    pub fn new() -> Self {
        return CameraFinder {
            all_cameras: Vec::new()
        }
    }

    pub fn find_cameras(&mut self) -> Result<(),String> {
        for i in 0..10 {
            let mut cam = try!(highgui::VideoCapture::device(i));
            let opened = try!(highgui::VideoCapture::is_opened(&cam));
            if opened {
                println!("Camera {} exists", i);
                self.all_cameras.push(i);
            }
        }
        Ok(())
    }

    pub fn first_or_default_camera(&self) -> i32 {
        self.all_cameras[0]
    }
}
