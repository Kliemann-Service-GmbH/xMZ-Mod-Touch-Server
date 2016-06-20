#[derive(Debug,Eq,PartialEq)]
pub enum ZoneType {
    Stoerung,
    Schwellenwert,
}

#[derive(Clone, Debug, Eq, PartialEq)] // für die Initalisierung `vec![Alarmpunkt(false); 4]`
struct Alarmpunkt(bool);

#[derive(Debug,Eq,PartialEq)]
pub struct Zone {
    zone_type: ZoneType,
    alarmpunkte: Vec<Alarmpunkt>,
}

impl Zone {
    pub fn new(zone_type: ZoneType) -> Self {
        match zone_type {
            ZoneType::Stoerung => Zone {
                zone_type: ZoneType::Stoerung,
                alarmpunkte: vec![Alarmpunkt(false)],
            },
            ZoneType::Schwellenwert => Zone {
                zone_type: ZoneType::Schwellenwert,
                alarmpunkte: vec![Alarmpunkt(false); 4],
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use server::zone::{Alarmpunkt, Zone, ZoneType};

    #[test]
    fn zone_type_stoerung_hat_ein_alarmpunkt() {
        let zone = Zone::new(ZoneType::Stoerung);
        assert_eq!(zone.alarmpunkte.len(), 1);
    }

    #[test]
    fn zone_type_schwellenwert_hat_4_alarmpunkte() {
        let zone = Zone::new(ZoneType::Schwellenwert);
        assert_eq!(zone.alarmpunkte.len(), 4);
    }

    #[test]
    fn alarmpunkt_kann_gesetzt_werden() {
        let mut zone = Zone::new(ZoneType::Schwellenwert);
        assert_eq!(zone.alarmpunkte[1], Alarmpunkt(false));
        zone.alarmpunkte[1] = Alarmpunkt(true);
        assert_eq!(zone.alarmpunkte[1], Alarmpunkt(true));
    }
}
