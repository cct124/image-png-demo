use std::fs::File;

pub fn handel(path: &str) {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let outputInfo = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..outputInfo.buffer_size()];
    let info = reader.info();

    println!(
        "
    {:?}
    {:?}
    {:?}
    ",
        info,
        outputInfo,
        bytes.len()
    )
}
