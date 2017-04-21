# Serverteil der 'xMZ-Mod-Touch'-Platform [![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.svg?branch=master)](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server)

[Projektseite |][homepage]&nbsp;[Dokumentation |][doku]&nbsp;[Repo auf Github.com |][repo]

Dies ist der Hauptprozess der sehr früh, noch vor der GUI, gestartet wird.
Er steuert die Hardware (LEDs, Relais, Akku usw.) und händelt die angeschlossenen
Module und deren Sensoren.


# JSON Web Api
Übersicht über die möglichen URL's

* Index, komplette Server Datenstruktur wird als JSON Objekt zurück gegeben
    `curl http://localhost:3000`
    oder
    `curl http://localhost:3000/api/v1`
* Zonen abfragen
    * Komplette Zone
    `curl http://localhost:3000/api/v1/zones`
    * ZoneID als Parameter übergeben 
        * `http://localhost:3000/api/v1/zone/[:zone_id]`
        `curl http://localhost:3000/api/v1/zone/0`
* Kombisensoren einer Zone abfragen
    * Alle Kombisensoren
    `curl http://localhost:3000/api/v1/zone/0/kombisensors`
    * KombisensorID als weiteren Parameter mit übergeben
        * `http://localhost:3000/api/v1/zone/[:zone_id]/kombisensor/[:kombisensor_id]`
        `curl http://localhost:3000/api/v1/zone/0/kombisensor/0`
* Sensoren eines Kombisensors abfragen
    * Alle Sensoren des Kombisensors abfragen        
    `curl http://localhost:3000/api/v1/zone/0/kombisensor/0/sensors`
    * SensorID als weiteren Parameter mit übergeben
        * `http://localhost:3000/api/v1/zone/[:zone_id]/kombisensor/[:kombisensor_id]/sensor/[:sensor_id]`
        `curl http://localhost:3000/api/v1/zone/0/kombisensor/0/sensor/0`
* Ausnahmen des Servers erfragen
    `curl http://localhost:3000/api/v1/exceptions`


# Tests
Optional können auch die Tests aufgerufen werden.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cargo test
```


# Links

* https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
* Dokumentation http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server/xmz_server/index.html
* Command Line Parser https://github.com/kbknapp/clap-rs
* RA-GAS CO-NO2-Kombisensor-Mod https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod

[1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
[homepage]: http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server
[repo]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server
[doku]: http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server/xmz_server/index.html
