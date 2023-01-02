use image::{codecs::png::PngEncoder, ImageEncoder};

use crate::wlrbackend::BufferData;
//use std::io::{stdout, BufWriter};
pub fn write_to_file(bufferdata: BufferData) {
    let mut writer = std::fs::File::create("test.png").unwrap();
    //let frame_mmap = &mut bufferdata.frame_mmap.unwrap();
    PngEncoder::new(&mut writer)
        .write_image(
            &bufferdata.frame_mmap.unwrap(),
            bufferdata.width,
            bufferdata.height,
            image::ColorType::Rgba8,
        )
        .unwrap()
}
