extern crate image;
extern crate svg;

use std::env;
use std::path::Path;

use svg::Document;
use svg::node::element::Rectangle;

use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name_in = &args[1];

    let img = image::open(name_in).unwrap();

    let (width, height) = img.dimensions();

    let mut document = Document::new()
        .set("viewBox", (0, 0, width, height));

    for (x, y, pixel) in img.pixels() {
        let [r, g, b, a] = &pixel.data;

        // if pixel is not clear, add it
        if a > &0 {
            let rect = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", 1)
            .set("height", 1)
            .set("fill", format!("rgba({}, {}, {}, {})", r, g, b, a));

            document = document.add(rect);
        }
    }

    let path = Path::new(name_in);
    let name_out = path.with_extension("svg");
    svg::save(&name_out, &document).unwrap();

    println!("{:?} saved.", &name_out);
}
