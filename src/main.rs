extern crate myr;

use myr::area;

fn main() {
    let mut area = area::create();

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
