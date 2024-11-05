pub mod gekkouga;
pub mod packun;
pub mod purin;
pub mod marth;
pub mod sheik;
pub mod jack;
pub mod zelda;
pub mod pit_bowarrow;
pub mod pit;
pub mod gaogaen;
pub mod littlemac;

pub fn install() {
    gekkouga::install();
    packun::install();
    purin::install();
    marth::install();
    sheik::install();
    jack::install();
    zelda::install();
    pit_bowarrow::install();
    pit::install();
    gaogaen::install();
    littlemac::install();
}