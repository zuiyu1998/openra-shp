use std::{fs::File, io::Read, path::Path};

use openra_shp::{Pal, Shp};

fn main() -> anyhow::Result<()> {
    let mut pal_file = File::open("./a_advpwr.pal")?;
    let mut pal_buf = Vec::new();

    pal_file.read_to_end(&mut pal_buf)?;

    let pal = Pal::new(&pal_buf)?;

    let mut shp_file = File::open("./a_advpwr.shp")?;
    let mut shp_buf = Vec::new();

    shp_file.read_to_end(&mut shp_buf)?;

    let shp = Shp::new(&shp_buf)?;

    for i in 0..shp.image_count {
        let img = shp.get_image(&pal, i);

        match img {
            Some(img) => {
                let file_name = format!("image_{}.png", i);
                img.save(&file_name).unwrap();
            }
            None => {}
        }
    }
    Ok(())
}
