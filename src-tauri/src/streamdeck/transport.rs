pub mod dummy;
pub mod streamdeck_rs;

use async_trait::async_trait;

#[async_trait]
pub trait Device: Send + Sync {
    fn close(&self);
    fn is_open(&self);
    fn connected(&self);
    async fn serial(&self) -> String;
    fn vendor_id(&self);
    fn product_id(&self);
    fn write_feature(&self, payload: String);
    fn read_feature(&self, report_id: String, length: String);
    fn write(&self, payload: String);
    fn read(&self, length: u8);
}

pub enum TransportType {
    // Dummy(),
    StreamdeckRs(),
}

#[async_trait]
pub(crate) trait Transport: Send + Sync {
    async fn enumerate(&mut self) -> Vec<Box<dyn Device>>;
}
