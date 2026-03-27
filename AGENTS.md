# Mundi Project Context

## Project Overview
Mundi is a geography learning application that quizzes users on map regions using interactive SVG maps. It starts with Spain (autonomous communities and provinces) and is designed for multi-country extensibility. Users click on map regions to answer quiz questions, with 3 attempts per region, color feedback (green=correct, red=wrong), and session + all-time statistics.

## Technology Stack
- **Language**: Rust
- **UI Framework**: GTK4 4.14+
- **Design**: libadwaita 1.5+ (Adwaita design system)
- **Build System**: Meson (production), Cargo (development)
- **Architecture**: GTK Builder with XML UI templates
- **Map Rendering**: GskPath::parse() with snapshot append_fill/append_stroke (no Cairo)
- **Hit Detection**: GskPath in_fill with FillRule::Winding
- **Settings**: GSettings for session and all-time stats

## Application Structure
- **Navigation**: Country list → Exercise list → Quiz view (AdwNavigationView)
- **Registry**: Data-driven design — countries and exercises defined in `registry.rs`
- **SVG Maps**: Simple `M L Z` path data with region name IDs, stored as GResources
- **Quiz Logic**: Random order, 3 attempts per region, translatable region names via gettext

## Development Workflow

### Quick Commands
```bash
# Development/testing
glib-compile-schemas data/
GSETTINGS_SCHEMA_DIR=data cargo run

# Production build
meson setup builddir
meson compile -C builddir
```

### Pre-commit Requirements
- **ALWAYS run `cargo fmt` before committing**
- Run `cargo clippy` to check for warnings
- Ensure code compiles without errors

## Release Process

### Version Bump
1. Update version in `Cargo.toml`
2. Update version in `meson.build`
3. Add release entry in metainfo XML with version, date, and changelog

### Creating Release
```bash
cargo fmt
cargo update -p mundi
git commit -am "Release X.Y.Z"
git tag vX.Y.Z
git push && git push --tags
```

## Code Conventions
- Follow Rust standard conventions (rustfmt, clippy)
- Use GTK4/Adwaita patterns for UI components
- Embed UI resources using GResource
- Separate UI templates (XML) from logic (Rust)
- All user-visible strings must be translatable via gettext
- Region names are translatable (SVG path id → gettext lookup)

## Adding New Countries/Exercises
1. Create SVG map in `resources/maps/<country>/` with region name IDs on `<path>` elements
2. Add region name constants in `region_names.rs`
3. Add exercises and country entry in `registry.rs`
4. SVG paths must use simple absolute `M L Z` coordinates (no curves, no transforms)
5. Enclaves/exclaves and islands are additional `M...L...Z` subpaths within the same `<path>` element

## Adding Capitals Exercises
Capitals exercises use `ExerciseKind::Capitals` and have a different SVG and quiz structure:

1. **SVG format**: Background region outlines use `_bg_` prefixed IDs (e.g. `_bg_Andalucía`), clickable capital dots are small 1×1 `<path>` squares with the capital name as ID (e.g. `id="Seville" d="M 170,440 L 171,440 L 171,441 L 170,441 Z"`)
2. **Region names**: Tuples are `(capital_name, region_name)` — e.g. `(N_("Seville"), N_("Andalusia"))`
3. **Quiz prompt**: Shows the capital name (e.g. "Select: Seville"), user clicks the correct dot on the map
4. **Discovery mode**: Shows "Seville, capital of Andalusia" when clicking a dot
5. **Dual capitals**: Use the `alternates` field on `MapExercise` to map alternate IDs to a primary ID — e.g. `alternates: &[("Las Palmas", "Santa Cruz de Tenerife")]` makes either Canary Islands capital dot count as correct

## Icon Design
- Application icon: `io.github.nacho.mundi.svg`
- **MUST follow GNOME HIG palette**: https://developer.gnome.org/hig/reference/palette.html

## Dependencies Management
- Core dependencies: GTK4, libadwaita, Rust toolchain, Meson
- Keep dependencies minimal and well-justified
- Prefer stable, well-maintained crates

## AI Assistant Guidelines
- Prioritize GTK4/Adwaita best practices
- Suggest modern Rust patterns appropriate for GUI applications
- Consider both development (Cargo) and production (Meson) build workflows
- Focus on GNOME HIG compliance for UI suggestions
- Keep code concise and maintainable
- Always remind about running `cargo fmt` before commits
- No Cairo — use GskPath for all map rendering and hit detection
