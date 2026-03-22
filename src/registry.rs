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
        title_msgid: "Autonomous Communities",
        svg_resource: "/io/github/nacho/mundi/maps/spain/communities.svg",
        regions: crate::region_names::SPAIN_COMMUNITIES,
    },
    MapExercise {
        id: "provinces",
        country_id: "spain",
        title_msgid: "Provinces",
        svg_resource: "/io/github/nacho/mundi/maps/spain/provinces.svg",
        regions: crate::region_names::SPAIN_PROVINCES,
    },
];

static ITALY_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "regions",
    country_id: "italy",
    title_msgid: "Regions",
    svg_resource: "/io/github/nacho/mundi/maps/italy/regions.svg",
    regions: crate::region_names::ITALY_REGIONS,
}];

static WORLD_EXERCISES: &[MapExercise] = &[
    MapExercise {
        id: "continents",
        country_id: "world",
        title_msgid: "Continents",
        svg_resource: "/io/github/nacho/mundi/maps/world/continents.svg",
        regions: crate::region_names::WORLD_CONTINENTS,
    },
    MapExercise {
        id: "africa-countries",
        country_id: "world",
        title_msgid: "Countries of Africa",
        svg_resource: "/io/github/nacho/mundi/maps/africa/countries.svg",
        regions: crate::region_names::AFRICA_COUNTRIES,
    },
    MapExercise {
        id: "america-countries",
        country_id: "world",
        title_msgid: "Countries of America",
        svg_resource: "/io/github/nacho/mundi/maps/america/countries.svg",
        regions: crate::region_names::AMERICA_COUNTRIES,
    },
    MapExercise {
        id: "asia-countries",
        country_id: "world",
        title_msgid: "Countries of Asia",
        svg_resource: "/io/github/nacho/mundi/maps/asia/countries.svg",
        regions: crate::region_names::ASIA_COUNTRIES,
    },
    MapExercise {
        id: "europe-countries",
        country_id: "world",
        title_msgid: "Countries of Europe",
        svg_resource: "/io/github/nacho/mundi/maps/europe/countries.svg",
        regions: crate::region_names::EUROPE_COUNTRIES,
    },
    MapExercise {
        id: "oceania-countries",
        country_id: "world",
        title_msgid: "Countries of Oceania",
        svg_resource: "/io/github/nacho/mundi/maps/oceania/countries.svg",
        regions: crate::region_names::OCEANIA_COUNTRIES,
    },
];

static US_EXERCISES: &[MapExercise] = &[MapExercise {
    id: "states",
    country_id: "united_states",
    title_msgid: "States",
    svg_resource: "/io/github/nacho/mundi/maps/united_states/states.svg",
    regions: crate::region_names::US_STATES,
}];

pub fn countries() -> &'static [Country] {
    static COUNTRIES: &[Country] = &[
        Country {
            id: "world",
            name_msgid: "World",
            exercises: WORLD_EXERCISES,
        },
        Country {
            id: "italy",
            name_msgid: "Italy",
            exercises: ITALY_EXERCISES,
        },
        Country {
            id: "spain",
            name_msgid: "Spain",
            exercises: SPAIN_EXERCISES,
        },
        Country {
            id: "united_states",
            name_msgid: "United States",
            exercises: US_EXERCISES,
        },
    ];
    COUNTRIES
}
