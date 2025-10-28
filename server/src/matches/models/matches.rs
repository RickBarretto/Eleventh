use super::teams::Team;

#[derive(Debug, Clone)]
pub struct Match {}

impl Match {
    pub fn new(host: impl Into<String>) -> Created {
        Created { by: host.into() }
    }
}

#[derive(Debug, Clone)]
pub struct Created {
    pub by: String,
}

impl Created {
    pub fn join(self, guest: impl Into<String>) -> Option<Paired> {
        Paired::new(self.by, guest)
    }
}

#[derive(Debug, Clone)]
pub struct Paired {
    pub host: String,
    pub host_team: Option<Team>,
    pub guest: String,
    pub guest_team: Option<Team>,
}

impl Paired {
    pub fn new(host: impl Into<String>, guest: impl Into<String>) -> Option<Self> {
        let host: String = host.into();
        let guest: String = guest.into();

        if host == guest {
            None
        } else {
            Some(Paired {
                host,
                guest,
                host_team: None,
                guest_team: None,
            })
        }
    }

    pub fn both_named(&self) -> bool {
        self.host_team.is_some() && self.guest_team.is_some()
    }

    pub fn name(&mut self, who: String, team: Team) {
        if self.host == who {
            self.host_team = Some(team);
        } else if self.guest == who {
            self.guest_team = Some(team);
        }
    }

    pub fn finish(&self) -> Option<Finished> {
        if !self.both_named() {
            return None;
        }

        let score: (usize, usize) = self.run_match();
        let winner: String = if score.0 > score.1 {
            self.host.clone()
        } else {
            self.guest.clone()
        };

        Some(Finished { winner, score })
    }

    pub fn run_match(&self) -> (usize, usize) {
        if !self.both_named() {
            return (0, 0);
        }

        let host = self.host_team.clone().unwrap();
        let guest = self.guest_team.clone().unwrap();

        host.play_with(&guest)
    }
}

#[derive(Debug, Clone)]
pub struct Finished {
    pub winner: String,
    pub score: (usize, usize),
}
