[![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.svg?branch=master)](https://travis-ci.org/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server)

Serverteil der 'xMZ-Mod-Touch'-Platform

Dies ist der Hauptprozess der sehr früh, noch vor der GUI, gestartet wird.
Er steuert die Hardware (LEDs, Relais, Akku usw.) und händelt die angeschlossenen
Module und deren Sensoren.

# Build, Compilation auf der 'xMZ-Mod-Touch'-Hardware
Die folgenden Befehle gehen davon aus das das Meta Git Repository
['xMZ-Mod-Touch-Software'][1] im HOME Verzeichnis ausgecheckt wurde.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cargo build --release
```

# Installation
## Programmdateien installieren

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cp target/release/xmz-server-bin /usr/bin/xmz-server
```


## Systemd Unit File anlegen
Dieser Schritt muss nur ein mal ausgeführt werden. Im Zweifel kann der Befehl aber
immer wieder aufgerufen werden (zum Beispiel im Update Fall).

```bash
cat <<EOF >/etc/systemd/system/xmz-mod-touch-server.service
#
# xMZ-Mod-Touch-Server systemd unit file
#
[Unit]
Description="Server Process der 'xMZ-Mod-Touch'-Platform"
After=multi-user.target

[Service]
ExecStart=/usr/bin/xmz-server &

[Install]
Alias=xmz-server.service
WantedBy=multi-user.target
EOF
```

Danach muss der service noch aktiviert ...

```bash
systemctl enable xmz-mod-touch-server.service
# systemctl daemon-reload # Dieser Befehl ist nur bei nachträglichen Änderungen am Unit File nötig!
```

... und gestartet werden.

```bash
systemctl restart xmz-mod-touch-server.service
```


# Update des Entwicklungssystems
Für ein Update muss zunächst die laufende Instanz beendet werden `systemctl stop`,
danach wird in das Verzeichnis mit dem Quellcode gewechselt, der aktuelle
Softwarestand mit `git pull` herunter geladen und anschließend die Software
gebaut.
Angeschlossen wird das Ganze indem die neu erstellen Binaries nach `/usr/bin`
kopiert werden und die Software neu gestartet wird `systemctl start`.

```bash
# systemctl daemon-reload # Evtl. wenn das systemd Unit File geändert wurde
systemctl stop xmz-mod-touch-server.service
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
git pull
cargo build --release
cp target/release/xmz-server-bin /usr/bin/xmz-server

systemctl start xmz-mod-touch-server.service
```


# Tests
Optional können auch die Tests aufgerufen werden.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cargo test
```


# Links

* https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
* Command Line Parser https://github.com/kbknapp/clap-rs

[1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
