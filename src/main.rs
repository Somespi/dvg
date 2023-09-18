use std::fs::metadata;
mod dvg;

fn main() {
    let width = 800;
    let height = 600;
    let dvg_image = dvg::DvgImage::new(width, height);

    dvg_image.save("dvg_image.dvg");
    

}
