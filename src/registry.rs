use crate::region_names::N_;
use gettextrs::gettext;

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

#[derive(Clone)]
pub struct MapExercise {
    pub id: &'static str,
    pub country_id: &'static str,
    title_msgid: &'static str,
    pub svg_resource: &'static str,
    pub regions: &'static [(&'static str, &'static str)],
}

impl MapExercise {
    pub fn title(&self) -> String {
        gettext(self.title_msgid)
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
        svg_resource: "/io/github/nacho/mundi/maps/spain/communities.svg",
        regions: crate::region_names::SPAIN_COMMUNITIES,
    },
    MapExercise {
        id: "provinces",
        country_id: "spain",
        title_msgid: N_("Provinces"),
        svg_resource: "/io/github/nacho/mundi/maps/spain/provinces.svg",
        regions: crate::region_names::SPAIN_PROVINCES,
    },
    MapExercise {
        id: "rivers",
        country_id: "spain",
        title_msgid: N_("Rivers"),
        svg_resource: "/io/github/nacho/mundi/maps/spain/rivers.svg",
        regions: crate::region_names::SPAIN_RIVERS,
    },
];

static ITALY_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "regions",
    country_id: "italy",
    title_msgid: N_("Regions"),
    svg_resource: "/io/github/nacho/mundi/maps/italy/regions.svg",
    regions: crate::region_names::ITALY_REGIONS,
}];

static WORLD_EXERCISES: &[MapExercise] = &[
    MapExercise {
        id: "continents",
        country_id: "world",
        title_msgid: N_("Continents"),
        svg_resource: "/io/github/nacho/mundi/maps/world/continents.svg",
        regions: crate::region_names::WORLD_CONTINENTS,
    },
    MapExercise {
        id: "africa-countries",
        country_id: "world",
        title_msgid: N_("Countries of Africa"),
        svg_resource: "/io/github/nacho/mundi/maps/africa/countries.svg",
        regions: crate::region_names::AFRICA_COUNTRIES,
    },
    MapExercise {
        id: "america-countries",
        country_id: "world",
        title_msgid: N_("Countries of America"),
        svg_resource: "/io/github/nacho/mundi/maps/america/countries.svg",
        regions: crate::region_names::AMERICA_COUNTRIES,
    },
    MapExercise {
        id: "asia-countries",
        country_id: "world",
        title_msgid: N_("Countries of Asia"),
        svg_resource: "/io/github/nacho/mundi/maps/asia/countries.svg",
        regions: crate::region_names::ASIA_COUNTRIES,
    },
    MapExercise {
        id: "europe-countries",
        country_id: "world",
        title_msgid: N_("Countries of Europe"),
        svg_resource: "/io/github/nacho/mundi/maps/europe/countries.svg",
        regions: crate::region_names::EUROPE_COUNTRIES,
    },
    MapExercise {
        id: "oceania-countries",
        country_id: "world",
        title_msgid: N_("Countries of Oceania"),
        svg_resource: "/io/github/nacho/mundi/maps/oceania/countries.svg",
        regions: crate::region_names::OCEANIA_COUNTRIES,
    },
];

static PORTUGAL_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "districts",
    country_id: "portugal",
    title_msgid: N_("Districts"),
    svg_resource: "/io/github/nacho/mundi/maps/portugal/districts.svg",
    regions: crate::region_names::PORTUGAL_DISTRICTS,
}];

static POLAND_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "voivodeships",
    country_id: "poland",
    title_msgid: N_("Voivodeships"),
    svg_resource: "/io/github/nacho/mundi/maps/poland/voivodeships.svg",
    regions: crate::region_names::POLAND_VOIVODESHIPS,
}];

static US_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "states",
    country_id: "united_states",
    title_msgid: N_("States"),
    svg_resource: "/io/github/nacho/mundi/maps/united_states/states.svg",
    regions: crate::region_names::US_STATES,
}];

pub fn countries() -> &'static [Country] {
    static COUNTRIES: &[Country] = &[
        Country {
            id: "world",
            name_msgid: N_("World"),
            exercises: WORLD_EXERCISES,
        },
        Country {
            id: "italy",
            name_msgid: N_("Italy"),
            exercises: ITALY_EXERCISES,
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
