# Mundi

A geography learning application for GNOME. Test your knowledge of world regions by clicking on an interactive map.

![GTK4](https://img.shields.io/badge/GTK-4-blue)
![License](https://img.shields.io/badge/license-GPL--3.0-green)

## Exercises

- **World** — Continents, Countries of Africa, America, Asia, Europe, Oceania; Capitals of Europe
- **France** — Regions
- **Germany** — States, State Capitals
- **Italy** — Regions
- **Japan** — Prefectures
- **Poland** — Voivodeships, Capitals of Voivodeships
- **Portugal** — Districts
- **Spain** — Autonomous Communities, Capitals of Autonomous Communities, Provinces, Rivers; Galicia: Provinces
- **United States** — States
- **India** - States

## How to Play

1. Pick a country and an exercise
2. The app asks you to find a region on the map
3. Click on the correct region — you get 3 attempts per question
4. Correct answers turn green, wrong ones turn red
5. Sound effects for correct and wrong answers (can be disabled in Preferences)

## Building

Mundi is written in Rust using GTK 4 and libadwaita.

```sh
glib-compile-schemas data/
GSETTINGS_SCHEMA_DIR=data cargo run
```
## Sound Sources

- Correct/wrong sound effects: [freedesktop sound theme](https://freedesktop.org/wiki/Specifications/sound-theme-spec/) (CC BY-SA)
- Quiz background music: [quiz game music loop BPM 90](https://freesound.org/people/portwain/sounds/220060/) by portwain (CC0)

## Map Sources

- World / Americas / Africa / Asia / Oceania: [BlankMap-World.svg](https://commons.wikimedia.org/wiki/File:BlankMap-World.svg) (public domain)
- Europe: [Blank map of Europe (with disputed regions)](https://commons.wikimedia.org/wiki/File:Blank_map_of_Europe_(with_disputed_regions).svg) (CC BY-SA 3.0)
- France: [simplemaps.com](https://simplemaps.com/svg/country/fr) (free for commercial and personal use), overseas territories from [Régions de France 2016 avec outremer.svg](https://commons.wikimedia.org/wiki/File:R%C3%A9gions_de_France_2016_avec_outremer.svg) (CC BY-SA 4.0)
- Germany: [simplemaps.com](https://simplemaps.com/svg/country/de#admin1) (free for commercial and personal use)
- Spain: [Mapa de España - Provincias.svg](https://commons.wikimedia.org/wiki/File:Mapa_de_Espa%C3%B1a_-_Provincias.svg) (CC BY-SA 4.0), [Blank map of Iberia.svg](https://commons.wikimedia.org/wiki/File:Blank_map_of_Iberia.svg) (CC BY-SA 4.0), [Spain, administrative divisions - Nmbrs - colored.svg](https://commons.wikimedia.org/wiki/File:Spain,_administrative_divisions_-_Nmbrs_-_colored.svg) (CC BY-SA 3.0, African coastline)
- Italy: [simplemaps.com](https://simplemaps.com/svg/country/it) (free for commercial and personal use)
- Japan: [Natural Earth Admin 1 - States, Provinces](https://www.naturalearthdata.com/downloads/10m-cultural-vectors/10m-admin-1-states-provinces/) (public domain)
- Poland: [simplemaps.com](https://simplemaps.com/svg/country/pl) (free for commercial and personal use)
- Portugal: [simplemaps.com](https://simplemaps.com/svg/country/pt) (free for commercial and personal use)
- United States: [simplemaps.com](https://simplemaps.com/svg/country/us) (free for commercial and personal use)
- India: [simplemaps.com](https://simplemaps.com/svg/country/in#admin1) (free for commercial and personal use)

## License

GPL-3.0-or-later
