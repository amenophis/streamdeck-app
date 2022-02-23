use crate::devices::device::Device;

struct Streamdeck {
    
}

impl Streamdeck {
    
}

impl Device for Streamdeck
{
    fn button_rows(&self) -> i32 { 3 }
    fn button_cols(&self) -> i32 { 5 }
}
