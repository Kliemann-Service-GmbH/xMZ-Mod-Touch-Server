

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ZoneType {
    STOERUNG,
    SCHWELLWERT,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Zone {
    zone_type: ZoneType,
}

impl Zone {
    pub fn new(zone_type: ZoneType) -> Self {
        match zone_type {
            ZoneType::STOERUNG => {
                Zone {
                    zone_type: zone_type,
                }
            }
            ZoneType::SCHWELLWERT => {
                Zone {
                    zone_type: zone_type,
                }
            }
        }
    }
}
