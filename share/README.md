Bilder und Grafiken des Projekts.

Die Logos werden für die Dokumentation `cargo doc` verwendet.


## Bilder zu Favicon konvertieren

Der folgende Befehl konvertiert das Bild `xmz-logo.png` in ein Multires Favicon.

```bash
convert xmz-logo.png   -bordercolor white -border 0 \
      \( -clone 0 -resize 16x16 \) \
      \( -clone 0 -resize 32x32 \) \
      \( -clone 0 -resize 48x48 \) \
      \( -clone 0 -resize 64x64 \) \
      -delete 0 -alpha off -colors 256 favicon.ico
```
