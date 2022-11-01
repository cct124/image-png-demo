mod example;

fn main() {
    // example::decoder_rgb::handel("pngs/rgb.png");
    // example::decoder_indexes::decoder_indexes("pngs/indexes.png");
    // example::decoder_rbg_apng::handel("pngs/apng_rgb.png");
    // example::decoder_indexes_apng::handel("pngs/apng_indexes.png");

    // example::encoder_rgb("pngs/rgb.png");
    // example::encoder_indexes::encoderIndexes("pngs/indexes.png");

    // example::imagequant_handle::png_handle("pngs/rgb.png");
    example::imagequant_handle::apng_handle("pngs/apng_test_1.png");
}
