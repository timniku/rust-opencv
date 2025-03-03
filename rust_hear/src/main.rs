use anyhow::Result;
use dotenv::dotenv;
use opencv::{core, highgui, imgcodecs, imgproc, objdetect, prelude::*, types, videoio};
use std::env;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;

    dotenv().ok();
    let haarcascades_file_type = env::var("HAARCASCADES_FILE")
        .expect("HAARCASCADES_FILE is not set in the environment variable.");

    let haarcascades_file_path = match &*haarcascades_file_type {
        "frontalface" => "haarcascades/haarcascade_frontalface_default.xml",
        "frontalface_alt" => "haarcascades/haarcascade_frontalface_alt.xml",
        "haarcascade_upperbody" => "haarcascades/haarcascade_upperbody.xml",
        "fullbody" => "haarcascades/haarcascade_fullbody.xml",
        _ => "haarcascades/haarcascade_frontalface_alt.xml",
    };

    let (xml, mut cam) = {
        (
            core::find_file(haarcascades_file_path, true, false)?,
            videoio::VideoCapture::new(2, videoio::CAP_ANY)?, // 0 is the default camera
        )
    };
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut face = objdetect::CascadeClassifier::new(&xml)?;
    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width == 0 {
            thread::sleep(Duration::from_secs(50));
            continue;
        }
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut reduced = Mat::default();
        imgproc::resize(
            &gray,
            &mut reduced,
            core::Size {
                width: 0,
                height: 0,
            },
            0.25f64,
            0.25f64,
            imgproc::INTER_LINEAR,
        )?;
        let mut faces = types::VectorOfRect::new();
        face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size {
                width: 30,
                height: 30,
            },
            core::Size {
                width: 0,
                height: 0,
            },
        )?;
        println!("faces: {}", faces.len());
        for face in faces {
            println!("face {:?}", face);
            let scaled_face = core::Rect {
                x: face.x * 4,
                y: face.y * 4,
                width: face.width * 4,
                height: face.height * 4,
            };
            imgproc::rectangle(
                &mut frame,
                scaled_face,
                core::Scalar::new(0f64, -1f64, -1f64, -1f64),
                1,
                8,
                0,
            )?;
            // Write image using OpenCV
            imgcodecs::imwrite("./tmp.png", &frame, &core::Vector::default())?;
        }
        highgui::imshow(window, &frame)?;
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}