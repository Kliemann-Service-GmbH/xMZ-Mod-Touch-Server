# Serverteil
## 'xMZ-Mod-Touch'-Platform [![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.svg?branch=master)](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server)

[Projektseite |][homepage]&nbsp;[Dokumentation |][doku]&nbsp;[Repo auf Github.com |][repo]

Dies ist der Hauptprozess der sehr früh, noch vor der GUI, gestartet wird.
Er steuert die Hardware (LEDs, Relais, Akku usw.) und händelt die angeschlossenen
Module und deren Sensoren.


## JSON Web Api
Übersicht über die möglichen URL's

|URL|Beschreibung|
|------------------------------------------------------------------------|------------------------------------------------------------------------|
|[http://localhost:3000](http://localhost:3000)|Index, komplette Server Datenstruktur wird als JSON Objekt zurück gegeben|
|[http://localhost:3000/api/v1](http://localhost:3000/api/v1)|Index, komplette Server Datenstruktur wird als JSON Objekt zurück gegeben|
|[http://localhost:3000/api/v1/zones](http://localhost:3000/api/v1/zones)|Alle Zonen abfragen|
|[http://localhost:3000/api/v1/zone/0](http://localhost:3000/api/v1/zone/0)|Eine Zone abfragen, ZoneID als Parameter übergeben|
|[http://localhost:3000/api/v1/zone/0/kombisensors](http://localhost:3000/api/v1/zone/0/kombisensors)|Alle Sensoren des Kombisensors abfragen|
|[http://localhost:3000/api/v1/zone/0/kombisensor/0](http://localhost:3000/api/v1/zone/0/kombisensor/0)||
|[http://localhost:3000/api/v1/zone/0/kombisensor/0/sensors](http://localhost:3000/api/v1/zone/0/kombisensor/0/sensors)|Alle Sensoren des Kombisensors abfragen|
|[http://localhost:3000/api/v1/zone/0/kombisensor/0/sensor/0](http://localhost:3000/api/v1/zone/0/kombisensor/0/sensor/0)|Ein Sensor abfragen, SensorID als weiteren Parameter mit übergeben|
|[http://localhost:3000/api/v1/exceptions](http://localhost:3000/api/v1/exceptions)|Ausnahmen des Servers erfragen|


## Tests
Optional können auch die Tests aufgerufen werden.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cargo test
```

## Links

* [https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software](https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software)
* Dokumentation [http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server/xmz_server/index.html](http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server/xmz_server/index.html)
* Command Line Parser [https://github.com/kbknapp/clap-rs](https://github.com/kbknapp/clap-rs)
* RA-GAS CO-NO2-Kombisensor-Mod [https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod](https://github.com/Kliemann-Service-GmbH/CO-NO2-Kombisensor-Mod)

##
Das verwendete [CSS File](http://s3.jfh.me/css/john-full.css) stammt von [http://s3.jfh.me/css/john-full.css](http://s3.jfh.me/css/john-full.css)

[1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
[homepage]: http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server
[repo]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server
[doku]: http://kliemann-service-gmbh.github.io/xMZ-Mod-Touch-Server/xmz_server/index.html
