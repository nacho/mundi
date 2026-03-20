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
    pub region_ids: &'static [&'static str],
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
        region_ids: crate::region_names::SPAIN_COMMUNITIES,
    },
    MapExercise {
        id: "provinces",
        country_id: "spain",
        title_msgid: "Provinces",
        svg_resource: "/io/github/nacho/mundi/maps/spain/provinces.svg",
        region_ids: crate::region_names::SPAIN_PROVINCES,
    },
];

static WORLD_EXERCISES: &[MapExercise] = &[
    MapExercise {
        id: "continents",
        country_id: "world",
        title_msgid: "Continents",
        svg_resource: "/io/github/nacho/mundi/maps/world/continents.svg",
        region_ids: crate::region_names::WORLD_CONTINENTS,
    },
    MapExercise {
        id: "europe-countries",
        country_id: "world",
        title_msgid: "Countries of Europe",
        svg_resource: "/io/github/nacho/mundi/maps/europe/countries.svg",
        region_ids: crate::region_names::EUROPE_COUNTRIES,
    },
];

pub fn countries() -> &'static [Country] {
    static COUNTRIES: &[Country] = &[
        Country {
            id: "world",
            name_msgid: "World",
            exercises: WORLD_EXERCISES,
        },
        Country {
            id: "spain",
            name_msgid: "Spain",
            exercises: SPAIN_EXERCISES,
        },
    ];
    COUNTRIES
}
