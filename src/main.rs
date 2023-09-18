
use image::Luma;
mod dvg;

fn main() {
    let width = 800;
    let height = 600;
    let mut dvg_image = dvg::DvgImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let intensity = ((x + y) * 2) as u8;
            dvg_image.put_pixel(&x, &y, Luma([intensity]));

        }
    }

    dvg_image.save("dvg_image2.dvg");

}
