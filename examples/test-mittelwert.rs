/// Durschnittswerte errechnen
///
/// Für die Mittelwertbildung (15min) müssen alle Messwerte aus diesem Zeitraum durch die Anzal der
/// Messwerte geteilt werden. Die Anzahl der Messwerte pro 15 Minuten ist unterschiedlich, je nach dem
/// wie schnell die Sensoren ausgelesen werden können.
///
extern crate chrono;

use ::chrono::{DateTime, UTC};


fn main() {
    // Die Werte (Messwerte) werden in eiem Tuppel, zusammen mit einem Timestamp gespeichert
    let mut values: Vec<(i32, DateTime<UTC>)> = vec![];

    // 10 Beispielwerte, einfach nur integers von 0..10
    for i in 0..10 {
        values.push((i, UTC::now()));
        // 500ms Pause, so dass wir pro Sekunde 2 Messwerte haben.
        // In der realen Implementierung ist das unterschiedlich, je nach dem wie schnell die Sensoren ausgelesen werden können
        ::std::thread::sleep(::std::time::Duration::from_millis(500));
    }

    // // Anzeige der Messwerte mit einem Index vorran, so das man die Position im Array erkennen kann.
    // // Unten steht der aktuellste Messwert mit seinem Zeitstempel
    // //
    // for (num, values) in values.clone().iter().enumerate() {
    //     println!("{:?}, {:?}", num, values);
    // };

    // // Anzeige einer Liste aller Timestamps, die Messwerte werden ignoriert.
    // // Die Lieste zeigt nur die vergangen Sekunden an
    // //
    // for (, timestamp) in values.clone() {
    //     println!("{}", UTC::now().signed_duration_since(timestamp).num_seconds());
    // }

    // Nur Messwerte der letzten `n` Sekunden behalten
    pub const MAX_SEC: i64 = 2;

    // Die [`binary_search_by_key()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.binary_search_by_key)
    // erhält als ersten Parameter die MAX_SEC Konstante, dann eine Referenz auf das Tuppel mit den (Messwerten, Zeitstempeln), und als letzten
    // Parameter die Bedingung die dem ersten Parameter vergleichen werden soll. In diesem Beispiel die vergangen Sekunden seit dem der Timestamp erstellt wurde
    //
    if let Ok(index) = values.binary_search_by_key(&MAX_SEC, |&(_, timestamp)| UTC::now().signed_duration_since(timestamp).num_seconds() ) {
        // Mit split off kann man nun den Vector teilen, es bleiben nur noch die (Messerte, Zeitstempel) der letzten MAX_SEC übrig.
        // **Dieser Rest wird nun wieder als values übernommen, alle anderen Werte werden verworfen.**
        //
        values = values.split_off(index);

        // // Nochmal rein guggen obs auch so ist ><
        // for (num, values) in values.clone().iter().enumerate() {
        //     println!("{:?}, {:?}", num, values);
        // };

        let num_values = values.len();
        let mut sum_values = 0;
        for &(value, _) in values.iter(){
            sum_values += value;
        }

        println!("num_values {:?}", num_values);
        println!("sum_values {:?}", sum_values);
        println!("durchschnitt {:?}", sum_values as f64 / num_values as f64);
    }


}
