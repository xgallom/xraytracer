use lodepng::encode24_file;

pub fn output(pixels: &[u8], width: usize, height: usize) {
    encode24_file("./output.png", pixels, width, height).expect("Failed writing output.");
}
