use crate::region_names::{N_, NC_};
use gettextrs::{gettext, pgettext};

#[derive(Clone)]
pub struct Country {
    #[allow(dead_code)]
    pub id: &'static str,
    name_msgid: &'static str,
    pub exercises: &'static [MapExercise],
}

impl Country {
    pub fn name(&self) -> String {
        gettext(self.name_msgid)
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum ExerciseKind {
    #[default]
    Standard,
    Capitals,
}

#[derive(Clone)]
pub struct MapExercise {
    pub id: &'static str,
    pub country_id: &'static str,
    title_msgid: &'static str,
    // Optional gettext message context, used to disambiguate identical
    // English titles that translate differently (e.g. "States" is
    // "Bundesländer" for Germany but "Bundesstaaten" for the US/India).
    title_context: Option<&'static str>,
    pub svg_resource: &'static str,
    pub regions: &'static [(&'static str, &'static str)],
    pub group: Option<&'static str>,
    pub kind: ExerciseKind,
    pub alternates: &'static [(&'static str, &'static str)],
}

impl MapExercise {
    pub fn title(&self) -> String {
        match self.title_context {
            Some(context) => pgettext(context, self.title_msgid),
            None => gettext(self.title_msgid),
        }
    }

    pub fn stats_path(&self) -> String {
        format!(
            "/io/github/nacho/mundi/stats/{}/{}/",
            self.country_id, self.id
        )
    }
}

static SPAIN_EXERCISES: &[MapExercise] = &[
    MapExercise {
        id: "communities",
        country_id: "spain",
        title_msgid: N_("Autonomous Communities"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/spain/communities.svg",
        regions: crate::region_names::SPAIN_COMMUNITIES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "community-capitals",
        country_id: "spain",
        title_msgid: N_("Capitals of Autonomous Communities"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/spain/community-capitals.svg",
        regions: crate::region_names::SPAIN_COMMUNITY_CAPITALS,
        group: None,
        kind: ExerciseKind::Capitals,
        alternates: &[("Las Palmas", "Santa Cruz de Tenerife")],
    },
    MapExercise {
        id: "provinces",
        country_id: "spain",
        title_msgid: N_("Provinces"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/spain/provinces.svg",
        regions: crate::region_names::SPAIN_PROVINCES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "rivers",
        country_id: "spain",
        title_msgid: N_("Rivers"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/spain/rivers.svg",
        regions: crate::region_names::SPAIN_RIVERS,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "galicia-provinces",
        country_id: "spain",
        title_msgid: N_("Provinces"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/galicia/provinces.svg",
        regions: crate::region_names::GALICIA_PROVINCES,
        group: Some(N_("Galicia")),
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
];

static FRANCE_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "regions",
    country_id: "france",
    title_msgid: N_("Regions"),
    title_context: None,
    svg_resource: "/io/github/nacho/mundi/maps/france/regions.svg",
    regions: crate::region_names::FRANCE_REGIONS,
    group: None,
    kind: ExerciseKind::Standard,
    alternates: &[],
}];

static GERMANY_EXERCISES: &[MapExercise] = &[
    MapExercise {
        id: "states",
        country_id: "germany",
        title_msgid: NC_("Germany", "States"),
        title_context: Some("Germany"),
        svg_resource: "/io/github/nacho/mundi/maps/germany/states.svg",
        regions: crate::region_names::GERMANY_STATES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "state-capitals",
        country_id: "germany",
        title_msgid: N_("State Capitals"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/germany/state-capitals.svg",
        regions: crate::region_names::GERMANY_STATE_CAPITALS,
        group: None,
        kind: ExerciseKind::Capitals,
        alternates: &[],
    },
];

static ITALY_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "regions",
    country_id: "italy",
    title_msgid: N_("Regions"),
    title_context: None,
    svg_resource: "/io/github/nacho/mundi/maps/italy/regions.svg",
    regions: crate::region_names::ITALY_REGIONS,
    group: None,
    kind: ExerciseKind::Standard,
    alternates: &[],
}];

static INDIA_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "states",
    country_id: "india",
    title_msgid: NC_("India", "States"),
    title_context: Some("India"),
    svg_resource: "/io/github/nacho/mundi/maps/india/states.svg",
    regions: crate::region_names::INDIA_STATES,
    group: None,
    kind: ExerciseKind::Standard,
    alternates: &[],
}];

static JAPAN_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "prefectures",
    country_id: "japan",
    title_msgid: N_("Prefectures"),
    title_context: None,
    svg_resource: "/io/github/nacho/mundi/maps/japan/prefectures.svg",
    regions: crate::region_names::JAPAN_PREFECTURES,
    group: None,
    kind: ExerciseKind::Standard,
    alternates: &[],
}];

static WORLD_EXERCISES: &[MapExercise] = &[
    MapExercise {
        id: "continents",
        country_id: "world",
        title_msgid: N_("Continents"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/world/continents.svg",
        regions: crate::region_names::WORLD_CONTINENTS,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "africa-countries",
        country_id: "world",
        title_msgid: N_("Countries of Africa"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/africa/countries.svg",
        regions: crate::region_names::AFRICA_COUNTRIES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "america-countries",
        country_id: "world",
        title_msgid: N_("Countries of America"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/america/countries.svg",
        regions: crate::region_names::AMERICA_COUNTRIES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "asia-countries",
        country_id: "world",
        title_msgid: N_("Countries of Asia"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/asia/countries.svg",
        regions: crate::region_names::ASIA_COUNTRIES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "europe-countries",
        country_id: "world",
        title_msgid: N_("Countries of Europe"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/europe/countries.svg",
        regions: crate::region_names::EUROPE_COUNTRIES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "europe-capitals",
        country_id: "world",
        title_msgid: N_("Capitals of Europe"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/europe/capitals.svg",
        regions: crate::region_names::EUROPE_CAPITALS,
        group: None,
        kind: ExerciseKind::Capitals,
        alternates: &[],
    },
    MapExercise {
        id: "oceania-countries",
        country_id: "world",
        title_msgid: N_("Countries of Oceania"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/oceania/countries.svg",
        regions: crate::region_names::OCEANIA_COUNTRIES,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
];

static PORTUGAL_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "districts",
    country_id: "portugal",
    title_msgid: N_("Districts"),
    title_context: None,
    svg_resource: "/io/github/nacho/mundi/maps/portugal/districts.svg",
    regions: crate::region_names::PORTUGAL_DISTRICTS,
    group: None,
    kind: ExerciseKind::Standard,
    alternates: &[],
}];

static POLAND_EXERCISES: &[MapExercise] = &[
    MapExercise {
        id: "voivodeships",
        country_id: "poland",
        title_msgid: N_("Voivodeships"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/poland/voivodeships.svg",
        regions: crate::region_names::POLAND_VOIVODESHIPS,
        group: None,
        kind: ExerciseKind::Standard,
        alternates: &[],
    },
    MapExercise {
        id: "voivodeship-capitals",
        country_id: "poland",
        title_msgid: N_("Capitals of Voivodeships"),
        title_context: None,
        svg_resource: "/io/github/nacho/mundi/maps/poland/voivodeship-capitals.svg",
        regions: crate::region_names::POLAND_VOIVODESHIP_CAPITALS,
        group: None,
        kind: ExerciseKind::Capitals,
        alternates: &[
            ("Toruń", "Bydgoszcz"),
            ("Zielona Góra", "Gorzów Wielkopolski"),
        ],
    },
];

static US_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "states",
    country_id: "united_states",
    title_msgid: NC_("United States", "States"),
    title_context: Some("United States"),
    svg_resource: "/io/github/nacho/mundi/maps/united_states/states.svg",
    regions: crate::region_names::US_STATES,
    group: None,
    kind: ExerciseKind::Standard,
    alternates: &[],
}];

pub fn countries() -> &'static [Country] {
    static COUNTRIES: &[Country] = &[
        Country {
            id: "world",
            name_msgid: N_("World"),
            exercises: WORLD_EXERCISES,
        },
        Country {
            id: "france",
            name_msgid: N_("France"),
            exercises: FRANCE_EXERCISES,
        },
        Country {
            id: "germany",
            name_msgid: N_("Germany"),
            exercises: GERMANY_EXERCISES,
        },
        Country {
            id: "india",
            name_msgid: N_("India"),
            exercises: INDIA_EXERCISES,
        },
        Country {
            id: "italy",
            name_msgid: N_("Italy"),
            exercises: ITALY_EXERCISES,
        },
        Country {
            id: "japan",
            name_msgid: N_("Japan"),
            exercises: JAPAN_EXERCISES,
        },
        Country {
            id: "poland",
            name_msgid: N_("Poland"),
            exercises: POLAND_EXERCISES,
        },
        Country {
            id: "portugal",
            name_msgid: N_("Portugal"),
            exercises: PORTUGAL_EXERCISES,
        },
        Country {
            id: "spain",
            name_msgid: N_("Spain"),
            exercises: SPAIN_EXERCISES,
        },
        Country {
            id: "united_states",
            name_msgid: N_("United States"),
            exercises: US_EXERCISES,
        },
    ];
    COUNTRIES
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn country(id: &str) -> &'static Country {
        countries()
            .iter()
            .find(|c| c.id == id)
            .unwrap_or_else(|| panic!("country '{id}' not found in registry"))
    }

    fn exercise(country_id: &str, exercise_id: &str) -> &'static MapExercise {
        country(country_id)
            .exercises
            .iter()
            .find(|e| e.id == exercise_id)
            .unwrap_or_else(|| panic!("exercise '{exercise_id}' not found for '{country_id}'"))
    }

    /// Reads an SVG resource from the repo (svg_resource is a GResource path
    /// like "/io/github/nacho/mundi/maps/germany/states.svg"; the on-disk file
    /// lives under resources/<same tail>).
    fn read_svg(svg_resource: &str) -> String {
        let tail = svg_resource
            .strip_prefix("/io/github/nacho/mundi/")
            .expect("unexpected svg_resource prefix");
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join(tail);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
    }

    fn assert_ids_present(svg: &str, ids: impl IntoIterator<Item = String>) {
        for id in ids {
            let needle = format!("id=\"{id}\"");
            assert!(
                svg.contains(&needle),
                "SVG is missing element with {needle}"
            );
        }
    }

    #[test]
    fn germany_states_registered() {
        let ex = exercise("germany", "states");
        assert!(ex.kind == ExerciseKind::Standard);
        assert_eq!(ex.regions.len(), 16, "Germany should have 16 states");
    }

    #[test]
    fn germany_states_names_unique() {
        let mut names: Vec<&str> = crate::region_names::GERMANY_STATES
            .iter()
            .map(|(svg_id, _)| *svg_id)
            .collect();
        names.sort_unstable();
        let count = names.len();
        names.dedup();
        assert_eq!(names.len(), count, "duplicate state svg_id found");
    }

    #[test]
    fn germany_states_svg_ids_present() {
        let ex = exercise("germany", "states");
        let svg = read_svg(ex.svg_resource);
        assert_ids_present(
            &svg,
            ex.regions.iter().map(|(svg_id, _)| svg_id.to_string()),
        );
    }

    #[test]
    fn germany_capitals_registered() {
        let ex = exercise("germany", "state-capitals");
        assert!(ex.kind == ExerciseKind::Capitals);
        assert_eq!(
            ex.regions.len(),
            16,
            "Germany should have 16 state capitals"
        );
    }

    #[test]
    fn germany_capitals_svg_ids_present() {
        let ex = exercise("germany", "state-capitals");
        let svg = read_svg(ex.svg_resource);
        // Each capital dot must be present by its own id.
        assert_ids_present(&svg, ex.regions.iter().map(|(cap, _)| cap.to_string()));
        // Each state must have a background outline (_bg_<State>).
        assert_ids_present(
            &svg,
            crate::region_names::GERMANY_STATES
                .iter()
                .map(|(state, _)| format!("_bg_{state}")),
        );
    }
}
