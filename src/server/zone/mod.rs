//! Alarmzonen, Schaltpunkte die die Relais und LED steuern


enum ZoneType {
    Stoerung,
    Schwellenwert,
}

#[derive(Clone)] // f+r die Initalisierung `vec![Alarmpunkt(false); 4]`
struct Alarmpunkt(bool);

struct Zone {
    zone_type: ZoneType,
    alarm_punkte: Vec<Alarmpunkt>,
}

impl Zone {
    fn new(zone_type: ZoneType) -> Self {
        match zone_type {
            ZoneType::Stoerung => Zone {
                zone_type: ZoneType::Stoerung,
                alarm_punkte: vec![Alarmpunkt(false)],
            },
            ZoneType::Schwellenwert => Zone {
                zone_type: ZoneType::Schwellenwert,
                alarm_punkte: vec![Alarmpunkt(false); 4],
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use server::zone::{Zone, ZoneType};

    #[test]
    fn zone_type_stoerung_hat_ein_alarmpunkt() {
        let zone = Zone::new(ZoneType::Stoerung);
        assert_eq!(zone.alarm_punkte.len(), 1);
    }

    #[test]
    fn zone_type_schwellenwert_hat_4_alarmpunkte() {
        let zone = Zone::new(ZoneType::Schwellenwert);
        assert_eq!(zone.alarm_punkte.len(), 4);
    }
}
