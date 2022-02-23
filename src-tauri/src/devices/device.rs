pub(crate) trait Device {
    fn button_rows(&self) -> i32;
    fn button_cols(&self) -> i32;
}
