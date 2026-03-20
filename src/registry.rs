use gettextrs::gettext;

#[derive(Clone)]
pub struct MapExercise {
    pub id: &'static str,
    pub country_id: &'static str,
    country_name_msgid: &'static str,
    title_msgid: &'static str,
    pub svg_resource: &'static str,
    pub region_ids: &'static [&'static str],
}

impl MapExercise {
    pub fn country_name(&self) -> String {
        gettext(self.country_name_msgid)
    }

    pub fn title(&self) -> String {
        gettext(self.title_msgid)
    }

    pub fn stats_path(&self) -> String {
        format!(
            "/io/github/nacho/learn-maps/stats/{}/{}/",
            self.country_id, self.id
        )
    }
}

pub fn exercises() -> Vec<MapExercise> {
    vec![
        MapExercise {
            id: "communities",
            country_id: "spain",
            country_name_msgid: "Spain",
            title_msgid: "Autonomous Communities",
            svg_resource: "/io/github/nacho/learn-maps/maps/spain/communities.svg",
            region_ids: crate::region_names::SPAIN_COMMUNITIES,
        },
        MapExercise {
            id: "provinces",
            country_id: "spain",
            country_name_msgid: "Spain",
            title_msgid: "Provinces",
            svg_resource: "/io/github/nacho/learn-maps/maps/spain/provinces.svg",
            region_ids: crate::region_names::SPAIN_PROVINCES,
        },
    ]
}
