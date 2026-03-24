# Mundi

A geography learning application for GNOME. Test your knowledge of world regions by clicking on an interactive map.

![GTK4](https://img.shields.io/badge/GTK-4-blue)
![License](https://img.shields.io/badge/license-GPL--3.0-green)

## Exercises

- **World** — Continents, Countries of Africa, America, Asia, Europe, Oceania
- **Italy** — Regions
- **Poland** — Voivodeships
- **Portugal** — Districts
- **Spain** — Autonomous Communities, Provinces, Rivers
- **United States** — States

## How to Play

1. Pick a country and an exercise
2. The app asks you to find a region on the map
3. Click on the correct region — you get 3 attempts per question
4. Correct answers turn green, wrong ones turn red

## Building

Mundi is written in Rust using GTK 4 and libadwaita.

```sh
glib-compile-schemas data/
GSETTINGS_SCHEMA_DIR=data cargo run
```

## Map Sources

- World / Americas / Africa / Asia / Oceania: [BlankMap-World.svg](https://commons.wikimedia.org/wiki/File:BlankMap-World.svg) (public domain)
- Europe: [Blank map of Europe (with disputed regions)](https://commons.wikimedia.org/wiki/File:Blank_map_of_Europe_(with_disputed_regions).svg) (CC BY-SA 3.0)
- Spain: [Mapa de España - Provincias.svg](https://commons.wikimedia.org/wiki/File:Mapa_de_Espa%C3%B1a_-_Provincias.svg) (CC BY-SA 4.0), [Blank map of Iberia.svg](https://commons.wikimedia.org/wiki/File:Blank_map_of_Iberia.svg) (CC BY-SA 4.0)
- Italy: [simplemaps.com](https://simplemaps.com/svg/country/it) (free for commercial and personal use)
- Poland: [simplemaps.com](https://simplemaps.com/svg/country/pl) (free for commercial and personal use)
- Portugal: [simplemaps.com](https://simplemaps.com/svg/country/pt) (free for commercial and personal use)
- United States: [simplemaps.com](https://simplemaps.com/svg/country/us) (free for commercial and personal use)

## License

GPL-3.0-or-later
