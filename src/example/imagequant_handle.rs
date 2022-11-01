use imagequant;
use png;
use rgb;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug)]
struct Frame {
    data: Vec<u8>,
    width: u32,
    height: u32,
    x_offset: u32,
    y_offset: u32,
    delay_num: u16,
    delay_den: u16,
    dispose_op: png::DisposeOp,
    blend_op: png::BlendOp,
}

impl Frame {
    pub fn new(
        data: Vec<u8>,
        width: u32,
        height: u32,
        x_offset: u32,
        y_offset: u32,
        delay_num: u16,
        delay_den: u16,
        dispose_op: png::DisposeOp,
        blend_op: png::BlendOp,
    ) -> Frame {
        Frame {
            data,
            width,
            height,
            x_offset,
            y_offset,
            delay_num,
            delay_den,
            dispose_op,
            blend_op,
        }
    }
}

pub fn png_handle(path: &str) {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    // let in_animation = reader.info().frame_control.is_some();

    let tmap = rgb::FromSlice::as_rgba(bytes);

    // println!("{:?}", tmap);

    let mut liq = imagequant::new();
    liq.set_speed(1).unwrap();
    liq.set_quality(0, 100).unwrap();

    // Describe the bitmap
    let mut img = liq
        .new_image(&tmap[..], info.width as usize, info.height as usize, 0.0)
        .unwrap();

    // The magic happens in quantize()
    let mut res = match liq.quantize(&mut img) {
        Ok(res) => res,
        Err(err) => panic!("Quantization failed, because: {:?}", err),
    };

    // Enable dithering for subsequent remappings
    res.set_dithering_level(1.0).unwrap();

    // You can reuse the result to generate several images with the same palette
    let (palette, pixels) = res.remapped(&mut img).unwrap();

    let mut rbg_palette: Vec<u8> = Vec::new();
    let mut trns: Vec<u8> = Vec::new();

    for f in palette.iter() {
        rbg_palette.push(f.r);
        rbg_palette.push(f.g);
        rbg_palette.push(f.b);
        trns.push(f.a);
    }
    // let rgb_pixels = rgb::FromSlice::as_rgba(pixels);

    let path = Path::new(r"pngs/indexes_image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    // println!("{:?}, {:?}", slice.len(), palette.len());

    let mut encoder = png::Encoder::new(w, info.width, info.height); // Width is 2 pixels and height is 1.
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_color(png::ColorType::Indexed);
    encoder.set_trns(trns);
    encoder.set_palette(rbg_palette);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap(); // Save
}

pub fn apng_handle(path: &str) {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut frames: Vec<Frame> = vec![];

    loop {
        let mut buf = vec![0; reader.output_buffer_size()];
        if let Result::Ok(output) = reader.next_frame(&mut buf) {
            let info = reader.info();
            let bytes = &buf[..output.buffer_size()];
            if let Some(control) = info.frame_control() {
                let frame = Frame::new(
                    bytes.to_vec(),
                    control.width,
                    control.height,
                    control.x_offset,
                    control.y_offset,
                    control.delay_num,
                    control.delay_den,
                    control.dispose_op,
                    control.blend_op,
                );
                frames.push(frame);
            }
        } else {
            break;
        }
    }

    let info = reader.info();

    let path = Path::new("pngs/apng_rbg_image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    if let Some(animation) = info.animation_control {
        encoder
            .set_animated(animation.num_frames, animation.num_plays)
            .unwrap();
        let mut writer = encoder.write_header().unwrap();

        for frame in frames.iter() {
            writer
                .set_frame_dimension(frame.width, frame.height)
                .unwrap();
            writer
                .set_frame_position(frame.x_offset, frame.y_offset)
                .unwrap();
            writer
                .set_frame_delay(frame.delay_num, frame.delay_den)
                .unwrap();
            writer.set_blend_op(frame.blend_op).unwrap();
            writer.set_dispose_op(frame.dispose_op).unwrap();
            writer.write_image_data(&frame.data).unwrap(); // Save
        }
    }
}
