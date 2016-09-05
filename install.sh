#!/bin/bash
# Dieses Script sollte nur ein einziges Mal aufgerufen werden! Alle weiteren
# Aufrufe bitte mit dem Script ./update.sh durchführen.

# Exit on error or variable unset
set -o errexit -o nounset


# Programmdateien installieren
cp -v ./target/release/xmz-server-bin /usr/bin/xmz-server

# Bibliotheken installieren
cp -rv ./target/release/build/libmodbus-sys-*/out/lib/* /usr/lib/

# Systemd Unit File anlegen
cat <<EOF | tee /etc/systemd/system/xmz-mod-touch-server.service
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

# Unit aktivieren ...
systemctl enable xmz-mod-touch-server.service

# Unit starten
systemctl restart xmz-mod-touch-server.service
