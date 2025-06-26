use bevy::prelude::*;

const TOTAL_LEVEL: usize = 2;

pub struct Level {
    path: String,
}

impl Level {
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

#[derive(Resource)]
pub struct Levels {
    all: Vec<Level>,
    current: usize,
}

impl Default for Levels {
    fn default() -> Self {
        Levels {
            all: vec![
                Level {
                    path: "assets/resources/1.jpg".to_string(),
                },
                Level {
                    path: "assets/resources/2.jpg".to_string(),
                },
            ],
            current: 0,
        }
    }
}

impl Levels {
    pub fn current_level(&self) -> &Level {
        self.all.get(self.current).unwrap()
    }

    pub fn next_level(&mut self) {
        self.current += 1;
        self.current %= TOTAL_LEVEL;
    }

    pub fn random_level(&mut self) {
        self.current = rand::random::<usize>() % TOTAL_LEVEL;
    }
}
