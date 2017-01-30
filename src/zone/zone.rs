#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(Eq, PartialEq)]
pub enum ZoneType {
    Stoerung,
    Alarmgruppe,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(Eq, PartialEq)]
pub struct Zone {
    zone_typ: ZoneType,
}

impl Zone {
    /// Erzeugt eine neue Zone
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let stoerung = Zone::new(ZoneType::Stoerung);
    /// ```
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let alarmgruppe1 = Zone::new(ZoneType::Stoerung);
    /// ```
    ///
    pub fn new(zone_typ: ZoneType) -> Self {
        match zone_typ {
            ZoneType::Stoerung => { Zone { zone_typ: zone_typ } }
            ZoneType::Alarmgruppe => { Zone { zone_typ: zone_typ } }
        }
    }

}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn stoerung() {
        let _stoerung = Zone::new(ZoneType::Stoerung);
    }

    #[test]
    fn alarmgruppe1() {
        let _alarmgruppe1 = Zone::new(ZoneType::Alarmgruppe);
    }
}
