#!/bin/bash
# Dieses Script sollte nur ein einziges Mal aufgerufen werden! Alle weiteren
# Aufrufe bitte mit dem Script ./update.sh durchführen.

# Exit on error or variable unset
set -o errexit -o nounset


Programmdateien installieren

cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cp -v ./target/release/xmz-server-bin /usr/bin/xmz-server

Bibliotheken installieren

cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Server
cp -rv ./target/release/build/libmodbus-sys-*/out/lib/* /usr/lib/

Systemd Unit File anlegen

Dieser Schritt muss nur ein mal ausgeführt werden. Im Zweifel kann der Befehl aber immer wieder aufgerufen werden (zum Beispiel im Update Fall).

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

Danach muss der service noch aktiviert ...

systemctl enable xmz-mod-touch-server.service
# systemctl daemon-reload # Dieser Befehl ist nur bei nachträglichen Änderungen am Unit File nötig!

... und gestartet werden.

systemctl restart xmz-mod-touch-server.service
