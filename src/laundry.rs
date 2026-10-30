use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::{Client, multipart};
use serde::{Deserialize, Serialize};

// Washer 1 is closest to the door, then 2, 3 and so on
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaundryMachine {
    Washer1,
    Washer2,
    Washer3,
    Washer4,
    Washer5,
    Dryer,
}

impl LaundryMachine {
    pub fn goods_id(&self) -> &'static str {
        match self {
            LaundryMachine::Washer1 => "1101161253",
            LaundryMachine::Washer2 => "1100860858",
            LaundryMachine::Washer3 => "1101161254",
            LaundryMachine::Washer4 => "1100860895",
            LaundryMachine::Washer5 => "1101161255",
            LaundryMachine::Dryer => "1101161256",
        }
    }

    pub async fn poll(&self, client: &Client) -> LaundryPollEvent {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        match self.check_availability(client).await {
            Ok(available) => LaundryPollEvent {
                machine: *self,
                available,
                success: true,
                error: None,
                request_time: timestamp,
            },
            Err(e) => LaundryPollEvent {
                machine: *self,
                available: false,
                success: false,
                error: Some(e.to_string()),
                request_time: timestamp,
            },
        }
    }

    pub async fn check_availability(
        &self,
        client: &Client,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let form = multipart::Form::new().text("goodsId", self.goods_id());

        let response = client
            .post("https://userapi.qiekj.com/goods/normal/details")
            .header("Accept", "application/json, text/plain, */*")
            .header("Origin", "https://h5.qiekj.com")
            .header("Referer", "https://h5.qiekj.com/")
            .header("phoneBrand", "DEFAULT")
            .header("uid", "undefined")
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(!response
            .text()
            .await?
            .contains("设备工作中，请更换设备使用"))
    }

    pub fn all() -> &'static [LaundryMachine; 6] {
        &[
            LaundryMachine::Washer1,
            LaundryMachine::Washer2,
            LaundryMachine::Washer3,
            LaundryMachine::Washer4,
            LaundryMachine::Washer5,
            LaundryMachine::Dryer,
        ]
    }

    pub fn id(&self) -> i32 {
        *self as i32
    }

    pub fn from_id(id: i32) -> Self {
        match id {
            0 => Self::Washer1,
            1 => Self::Washer2,
            2 => Self::Washer3,
            3 => Self::Washer4,
            4 => Self::Washer5,
            5 => Self::Dryer,
            _ => panic!("yo invalid id: {}", id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaundryPollEvent {
    machine: LaundryMachine,
    available: bool,
    success: bool,
    error: Option<String>,
    request_time: u64,
}

pub trait LaundryStore {
    fn save_poll_event(&self, poll_event: LaundryPollEvent) -> Result<(), StorageError>;

    fn load_poll_events(&self) -> Result<Vec<LaundryPollEvent>, StorageError>;
}

pub type StorageError = Box<dyn std::error::Error + Send + Sync>;
