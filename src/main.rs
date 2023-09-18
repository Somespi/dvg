use image::Luma;

mod dvg;

fn main () {
    let mut img = dvg::DvgImage::new(1000, 1000);
    for x in 100..900 {
        for y in 100..900 {
            
            let insitsty = if y % 2 == 0 { 255 } else { 122 };
            let pixel = Luma([insitsty]); 
    
            for i in 0..10 {
                img.put_pixel(&(x + i),& y, pixel);
            }
        }
    }

    img.save("./test.dvg");
    img.save_as_png(("./test.png"));

}