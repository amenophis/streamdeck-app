// use async_trait::async_trait;

// use super::{Device, Transport};

pub struct Dummy {}

impl Dummy {
    pub fn new() -> Self {
        Self {}
    }
}

// #[async_trait]
// impl Transport for Dummy {
//     async fn enumerate(&self) -> Vec<Device> {
//         Vec::new()
//     }

//     async fn connect(&self) -> bool {
//         true
//     }
// }
