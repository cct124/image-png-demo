use std::fs::File;

pub fn handel(path: &str) {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut bufs: Vec<Vec<u8>> = vec![];

    loop {
        let mut buf = vec![0; reader.output_buffer_size()];
        if let Result::Ok(output) = reader.next_frame(&mut buf) {
            let bytes = &buf[..output.buffer_size()];
            bufs.push(bytes.to_vec());
        } else {
            break;
        }
    }

    let info = reader.info();

    if let Some(palette) = &info.palette {
        println!("paletteLen: {}", palette.len())
    }

    println!(
        "
    {:?}
    {:?}
    {:?}
    ",
        info,
        bufs,
        bufs.len()
    )
}
