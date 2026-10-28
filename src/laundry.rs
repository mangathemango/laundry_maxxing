use std::time::Instant;

use reqwest::{Client, multipart};

// Washer 1 is closest to the door, then 2, 3 and so on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, Copy)]
pub struct LaundryState {
    pub machine: LaundryMachine,
    pub last_available: Option<Instant>,
    pub last_poll: Option<Instant>,   
    pub is_available: bool 
}

impl LaundryState {
    pub fn new(machine: LaundryMachine) -> Self {
        Self {
            machine,
            last_available: None,
            last_poll: None,
            is_available: true
        }
    }

    pub fn all() -> Vec<Self> {
        LaundryMachine::all()
            .map(Self::new)
            .into_iter()
            .collect()
    }

    pub async fn try_poll(&mut self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        match self.machine.check_availability(client).await? {
            true => {
                self.last_available = Some(Instant::now());
                self.is_available = true;
            }
            false => {
                self.is_available = false;
            }
        }
        self.last_poll = Some(Instant::now());

        Ok(())
    } 
}


