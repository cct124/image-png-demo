use png;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn encoderIndexes(path: &str) {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let outputInfo = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..outputInfo.buffer_size()];
    let info = reader.info();

    println!("{:?}", bytes.len());

    let path = Path::new(r"pngs/indexes_image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height); // Width is 2 pixels and height is 1.
    encoder.set_depth(png::BitDepth::Eight);

    // println!("{:?}", info);

    if let Some(trsn) = &info.trns {
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_trns(trsn.to_vec());

        println!("{:?}", trsn.to_vec().len())
    } else {
        encoder.set_color(png::ColorType::Rgba);
    };

    if let Some(palette) = &info.palette {
        encoder.set_palette(palette.to_vec());
        println!("{:?}", palette.to_vec().len())
    };

    // encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    // encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    // let source_chromaticities = png::SourceChromaticities::new(
    //     // Using unscaled instantiation here
    //     (0.31270, 0.32900),
    //     (0.64000, 0.33000),
    //     (0.30000, 0.60000),
    //     (0.15000, 0.06000),
    // );
    // encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&bytes).unwrap(); // Save
}
