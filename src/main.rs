extern crate myr;

use myr::phys::area::Area;

fn main() {
    let mut area = Area::new();

    area.tick();
    area.tick();
    area.tick();
    area.tick();
    area.tick();

    /*
    loop {
        area.tick();
    }
    */
}
