mod aerials;
mod normals;
mod specials;

pub fn install() {
    aerials::install();
    normals::install();
    specials::install();
}