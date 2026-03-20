use rand::seq::SliceRandom;

pub struct Quiz {
    regions: Vec<(String, String)>, // (id, name_msgid)
    current: usize,
    pub attempts_left: u32,
    pub session_correct: u32,
    pub session_total: u32,
}

impl Quiz {
    pub fn new(regions: &[(&str, &str)]) -> Self {
        let mut regions: Vec<(String, String)> = regions
            .iter()
            .map(|(id, name)| (id.to_string(), name.to_string()))
            .collect();
        regions.shuffle(&mut rand::rng());
        Quiz {
            regions,
            current: 0,
            attempts_left: 3,
            session_correct: 0,
            session_total: 0,
        }
    }

    pub fn current_id(&self) -> Option<&str> {
        self.regions.get(self.current).map(|(id, _)| id.as_str())
    }

    pub fn current_name(&self) -> Option<&str> {
        self.regions
            .get(self.current)
            .map(|(_, name)| name.as_str())
    }

    pub fn is_finished(&self) -> bool {
        self.current >= self.regions.len()
    }

    /// Returns true if correct
    pub fn answer(&mut self, region_id: &str) -> bool {
        if let Some(target) = self.current_id() {
            if region_id == target {
                self.session_correct += 1;
                self.session_total += 1;
                self.current += 1;
                self.attempts_left = 3;
                return true;
            }
        }
        self.attempts_left = self.attempts_left.saturating_sub(1);
        if self.attempts_left == 0 {
            self.session_total += 1;
            self.current += 1;
            self.attempts_left = 3;
        }
        false
    }

    pub fn session_percentage(&self) -> f64 {
        if self.session_total == 0 {
            0.0
        } else {
            (self.session_correct as f64 / self.session_total as f64) * 100.0
        }
    }
}
