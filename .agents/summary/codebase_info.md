# Codebase Information
<!-- metadata: type=overview, audience=ai-agents, scope=project-wide -->

## Project Identity

- **Name**: Mundi
- **Application ID**: `io.github.nacho.mundi`
- **Version**: 0.9.0
- **License**: GPL-3.0-or-later
- **Author**: Ignacio Casal Quinteiro
- **Repository**: https://github.com/nacho/mundi

## Purpose

Geography learning application for GNOME. Users explore interactive SVG maps in discovery mode and take timed quizzes identifying regions, countries, capitals, and rivers by clicking on the map.

## Technology Stack

| Layer | Technology | Version Constraint |
|-------|-----------|-------------------|
| Language | Rust | Edition 2024 |
| UI Framework | GTK4 | ≥ 4.14 |
| Design System | libadwaita | ≥ 1.5 |
| Build (production) | Meson | ≥ 0.59.0 |
| Build (development) | Cargo | — |
| Map Rendering | GskPath | (no Cairo) |
| Settings | GSettings (dconf) | — |
| i18n | gettext | — |
| Resources | GResource | — |
| Serialization | serde + serde_json | — |
| SVG Parsing | quick-xml | — |
| CI | GitHub Actions | — |

## Supported Languages (i18n)

Spanish (es), Galician (gl), Italian (it), Polish (pl)

## Directory Structure

```
mundi/
├── src/                    # Rust source (12 files)
├── resources/
│   ├── ui/                 # GTK Builder XML templates (4 files)
│   ├── maps/               # SVG map files (12 subdirectories, 22 maps)
│   ├── sounds/             # Audio files (3: correct, wrong, quiz-music)
│   ├── gtk/                # Menu definition (menus.ui)
│   ├── style.css           # Custom CSS (named colors for map states)
│   └── resources.gresource.xml
├── data/
│   ├── io.github.nacho.mundi.gschema.xml    # GSettings schema
│   ├── io.github.nacho.mundi.metainfo.xml   # AppStream metadata
│   ├── io.github.nacho.mundi.desktop.in     # Desktop entry
│   ├── icons/              # Application icon (SVG)
│   ├── screenshots/        # Store screenshots (PNG)
│   └── meson.build         # Data install rules
├── po/                     # Translations (4 languages + template)
├── .github/workflows/      # CI (ci.yml)
├── build.rs                # Cargo build script (config generation + GResource compilation)
├── Cargo.toml              # Rust dependencies
├── meson.build             # Meson build definition
└── AGENTS.md               # AI assistant context
```

## Source Files Overview

| File | Responsibility |
|------|---------------|
| `main.rs` | Entry point, i18n init, GResource registration |
| `application.rs` | `MundiApplication` — app lifecycle, actions, about/preferences dialogs |
| `window.rs` | `MundiWindow` — main window, country list, navigation, window state persistence |
| `registry.rs` | Data-driven country/exercise definitions (static data) |
| `region_names.rs` | Translatable region name constants (`N_()` macro) |
| `map_widget.rs` | `MapWidget` — SVG parsing, GskPath rendering, hit detection, region state management |
| `map_exercise_view.rs` | `MapExerciseView` — quiz/discovery mode controller, timer, sound integration |
| `quiz.rs` | `Quiz` — quiz state machine (random order, 3 attempts, scoring) |
| `quiz_results_view.rs` | `QuizResultsView` — score display, leaderboard UI |
| `leaderboard.rs` | `Leaderboard` — JSON persistence, ranking logic (top 50) |
| `sound_player.rs` | `SoundPlayer` — GtkMediaFile wrapper for correct/wrong/music sounds |
| `preferences_dialog.rs` | `PreferencesDialog` — sound effects toggle bound to GSettings |
| `config.rs.in` | Build-time config template (version, app ID, paths) |

## Map Coverage

| Country/Region | Exercises |
|---------------|-----------|
| World | Continents |
| Africa | Countries |
| America | Countries |
| Asia | Countries |
| Europe | Countries, Capitals |
| Italy | Regions |
| Poland | Voivodeships, Capitals of Voivodeships |
| Portugal | Districts |
| Spain | Autonomous Communities, Capitals of Autonomous Communities, Provinces, Rivers, Lakes, Mountain Ranges, Peaks and Volcanoes, Seas, Islands and Archipelagos, Plains and Hills, Gulfs and Peninsulas |
| Spain/Galicia | Provinces |
| United States | States |
