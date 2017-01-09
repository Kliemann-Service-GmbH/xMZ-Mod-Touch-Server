// use errors::*;


#[derive(Debug, Eq, PartialEq)]
pub enum AlarmgruppenTyp {
    Stoerung,
    Zone,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Alarmgruppe {
    gruppen_typ: AlarmgruppenTyp,
}

impl Alarmgruppe {
    /// Erzeugt eine neue Alarmgruppe
    ///
    /// # Examples
    /// ```
    /// use xmz_server::*;
    ///
    /// let stoerung = Alarmgruppe::new(AlarmgruppenTyp::Stoerung);
    /// ```
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let zone1 = Alarmgruppe::new(AlarmgruppenTyp::Stoerung);
    /// ```
    ///
    pub fn new(gruppen_typ: AlarmgruppenTyp) -> Self {
        match gruppen_typ {
            AlarmgruppenTyp::Stoerung => { Alarmgruppe { gruppen_typ: gruppen_typ } }
            AlarmgruppenTyp::Zone => { Alarmgruppe { gruppen_typ: gruppen_typ } }
        }
    }

}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn stoerung() {
        let _stoerung = Alarmgruppe::new(AlarmgruppenTyp::Stoerung);
    }

    #[test]
    fn zone1() {
        let _zone1 = Alarmgruppe::new(AlarmgruppenTyp::Zone);
    }
}
