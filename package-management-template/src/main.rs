use crate::resources::resourcea;
use crate::resources::resourceb;

pub mod resources;
fn main() {
    resourcea::foo();
    resourceb::bar();
}
