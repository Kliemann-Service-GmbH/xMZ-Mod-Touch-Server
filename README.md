Serverteil der 'xMZ-Mod-Touch'-Platform

Dies ist der Hauptprozess der sehr früh, noch vor der GUI, gestartet wird.
Er steuert die Hardware (LEDs, Relais, Akku usw.) und händelt die angeschlossenen
Module und deren Sensoren.

# Build
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

```bash
cat <<EOF >/etc/systemd/system/xmz-mod-touch-server.service
[Unit]
Description="Server Process of the 'xMZ-Mod-Touch'-Platform"
After=multi-user.target

[Service]
ExecStart=/usr/bin/xmz-server &

[Install]
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





# Tests

Optional können auch die Tests aufgerufen werden.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cargo build --release
```



# Links

* https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
* Command Line Parser https://github.com/kbknapp/clap-rs


[1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
