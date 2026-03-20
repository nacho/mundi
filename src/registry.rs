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

pub fn countries() -> &'static [Country] {
    static COUNTRIES: &[Country] = &[Country {
        id: "spain",
        name_msgid: "Spain",
        exercises: SPAIN_EXERCISES,
    }];
    COUNTRIES
}
