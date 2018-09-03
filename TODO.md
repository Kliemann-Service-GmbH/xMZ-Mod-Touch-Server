* COPYING nach LICENSE
* Projektname xmz-mod-touch-server


# Fehler

- wird in der Konfig der NO2 Sensor ausgetragen dann findet keine Sensorauswertung mehr statt
- Wird das config Bit (enabled) für den NO2 Sensor gesetzt, deaktiviert den Sensor, dann erscheint der NO2 Sensor immernoch in der GUI (GUI Problem?)

# Primäre Aufgaben
Diese Aufgaben sind sehr wichtig.

* Fehlerbehandlung in `src/json_api.rs` verbessern
* Commit `7464935f` Software Stand übernehmen
* yocto/ poky builds
    * xmz_server
    * xmz_gui
* Konfigurationsdatei
    * einlesen
    * schreiben


# Sekundäre Aufgaben
Aufgaben die nicht ganz so wichtig für die Hauptfunkionen der Software sind.
Kosmetische Verbesseungen zum Beispiel.

* `src/json_api.rs` dokumentieren.
