use image::{codecs::jpeg::JpegEncoder, RgbImage, RgbaImage};
use zune_core::{bytestream::ZCursor, options::DecoderOptions};
use zune_jpeg::JpegDecoder;

#[test]
fn decode_rgb565() {
    let src_jpeg = include_bytes!("../image.jpg");
    let image = image::load_from_memory(src_jpeg).unwrap().to_rgb8();
    let mut buffer = vec![];
    let mut encoder = JpegEncoder::new(&mut buffer);
    encoder.encode_image(&image).unwrap();

    println!("jpeg压缩之后:{}", buffer.len());
    
    let mut decoder = JpegDecoder::new_with_output_colorspace(ZCursor::new(&buffer), zune_core::colorspace::ColorSpace::RGB565);
    let options = DecoderOptions::new_fast()
        .jpeg_set_out_colorspace(zune_core::colorspace::ColorSpace::RGB565);
    decoder.set_options(options);
    let rgb565 = decoder.decode().unwrap();
    println!("input_colorspace={:?}", decoder.input_colorspace());
    println!("output_colorspace={:?}", decoder.output_colorspace());
    let width = decoder.info().unwrap().width;
    let height = decoder.info().unwrap().height;
    println!("len:{} {width}x{height}", rgb565.len());

    std::fs::write("image.rgb565", &rgb565).unwrap();
}


#[test]
fn decode_rgb() {
    let src_jpeg = include_bytes!("../image.jpg");
    let image = image::load_from_memory(src_jpeg).unwrap().to_rgb8();
    let mut buffer = vec![];
    let mut encoder = JpegEncoder::new(&mut buffer);
    encoder.encode_image(&image).unwrap();

    println!("jpeg压缩之后:{}", buffer.len());
    
    let mut decoder = JpegDecoder::new(ZCursor::new(&buffer));
    let options = DecoderOptions::new_fast();
    decoder.set_options(options);
    let rgb = decoder.decode().unwrap();
    println!("input_colorspace={:?}", decoder.input_colorspace());
    println!("output_colorspace={:?}", decoder.output_colorspace());
    let width = decoder.info().unwrap().width;
    let height = decoder.info().unwrap().height;
    println!("len:{} {width}x{height}", rgb.len());

    let image = RgbImage::from_raw(width as u32, height as u32, rgb).unwrap();
    image.save("image.png").unwrap();
}