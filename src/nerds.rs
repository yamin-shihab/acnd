use console_engine::Color;

// Array of nerds
pub const NERDS: [&Nerd; 4] = [&JOE, &ISAAC, &WILLIAM, &SUZIE];

// Color of selected nerd in game and menu
pub const CURRENT_NERD_COLOR: Color = Color::Green;
pub const WAITING_NERD_COLOR: Color = Color::Red;

// Values that change damage done
pub const BASE_MULTIPLIER: f64 = 1.0;
pub const CRITICAL_CHANCE: u32 = 20;
pub const CRITICAL_MULTIPLIER: f64 = 2.0;

// Used to represent the two players
pub type Nerds = [Nerd; 2];

// Balanced nerd
pub const JOE: Nerd = Nerd::new(
    "Joe",
    200,
    [
        Action::Damage(ActionStats::new("Slap", 30)),
        Action::Heal(ActionStats::new("Band-Aid", 20)),
        Action::Weaken(ActionStats::new("Pinch", 0.20)),
        Action::Strengthen(ActionStats::new("Khan Academy", 0.20)),
    ],
    " / \\
| \" |
 \\o/
  |
 /|\\
/ | \\
 / \\
/   \\",
);

// Offensive nerd
pub const ISAAC: Nerd = Nerd::new(
    "Isaac",
    100,
    [
        Action::Damage(ActionStats::new("Meter Ruler Katana", 60)),
        Action::Heal(ActionStats::new("Self Confidence/Motivation", 20)),
        Action::Weaken(ActionStats::new("Threaten with Scissors", 0.40)),
        Action::Strengthen(ActionStats::new("Steroids", 0.20)),
    ],
    " (\")
 \\-/
  |
\\/|\\/
  |
  |
 / \\
 | |",
);

// Defensive nerd
pub const WILLIAM: Nerd = Nerd::new(
    "William",
    400,
    [
        Action::Damage(ActionStats::new("Curse/Swear Words", 30)),
        Action::Heal(ActionStats::new("Meditation", 10)),
        Action::Weaken(ActionStats::new("Intimidating Stare", 0.10)),
        Action::Strengthen(ActionStats::new("Inflatable Dumbbells", 0.20)),
    ],
    "   __
  /''\\
\\ \\()/ /
 \\/  \\/
 |    |
  \\  /
  //\\\\
_//  \\\\_",
);

// Healer nerd
pub const SUZIE: Nerd = Nerd::new(
    "Suzie",
    200,
    [
        Action::Damage(ActionStats::new("Insult", 15)),
        Action::Heal(ActionStats::new("First Aid Kit", 40)),
        Action::Weaken(ActionStats::new("Threaten to Tell Teacher", 0.10)),
        Action::Strengthen(ActionStats::new("Watch Dhar Mann Video", 0.40)),
    ],
    " //\"\\\\
/ \\~/ \\
   |
  /|\\
  \\|/
   |
  / \\
 /   \\",
);

// A character/player with their stats
#[derive(Copy, Clone)]
pub struct Nerd {
    pub name: &'static str,
    pub health: i32,
    pub multiplier: f64,
    pub actions: [Action; 4],
    pub sprite: &'static str,
}

impl Nerd {
    // Creates a new nerd
    pub const fn new(
        name: &'static str,
        health: i32,
        actions: [Action; 4],
        sprite: &'static str,
    ) -> Self {
        Self {
            name,
            health,
            multiplier: BASE_MULTIPLIER,
            actions,
            sprite,
        }
    }

    // Returns the equation to be answered, the answer to it, and whether it was a crtiical
    pub fn equation(&self, action: usize, nerd: &Nerd) -> (String, f64, bool) {
        let critical = Self::critical();
        match self.actions[action] {
            Action::Damage(stats) => (
                format!(
                    "{} - ({} * {} * {})",
                    nerd.health, stats.value, self.multiplier, critical
                ),
                ((nerd.health as f64 - (stats.value as f64 * self.multiplier * critical)) * 100.0)
                    .trunc()
                    / 100.0,
                critical == CRITICAL_MULTIPLIER,
            ),
            Action::Heal(stats) => (
                format!(
                    "{} + ({} * {} * {})",
                    self.health, stats.value, self.multiplier, critical
                ),
                ((self.health as f64 + (stats.value as f64 * self.multiplier * critical)) * 100.0)
                    .trunc()
                    / 100.0,
                critical == CRITICAL_MULTIPLIER,
            ),
            Action::Weaken(stats) => (
                format!(
                    "{} - ({} * {} * {} * {})",
                    nerd.multiplier, nerd.multiplier, stats.value, self.multiplier, critical
                ),
                ((nerd.multiplier - (nerd.multiplier * stats.value * self.multiplier * critical))
                    * 100.0)
                    .trunc()
                    / 100.0,
                critical == CRITICAL_MULTIPLIER,
            ),
            Action::Strengthen(stats) => (
                format!(
                    "{} + ({} * {} * {} * {})",
                    self.multiplier, self.multiplier, stats.value, self.multiplier, critical
                ),
                ((self.multiplier + (self.multiplier * stats.value * self.multiplier * critical))
                    * 100.0)
                    .trunc()
                    / 100.0,
                critical == CRITICAL_MULTIPLIER,
            ),
        }
    }

    // Uses the given action index
    pub fn use_action(
        &mut self,
        action: usize,
        value: f64,
        critical: bool,
        nerd: &mut Nerd,
    ) -> String {
        match self.actions[action] {
            Action::Damage(_) => nerd.health = value as i32,
            Action::Heal(_) => self.health = value as i32,
            Action::Weaken(_) => nerd.multiplier = value,
            Action::Strengthen(_) => self.multiplier = value,
        }
        self.action_message(action, critical, nerd)
    }

    // Returns a critical hit multiplier
    fn critical() -> f64 {
        let rand = fastrand::u32(0..100);
        if rand < CRITICAL_CHANCE {
            return CRITICAL_MULTIPLIER;
        }
        1.0
    }

    // Returns a message to be displayed as a result of an action
    fn action_message(&self, action: usize, critical: bool, nerd: &Nerd) -> String {
        format!(
            "{} used {} against {}{}",
            self.name,
            self.actions[action].name(),
            nerd.name,
            if critical { " (CRITICAL!!!)" } else { "" }
        )
    }
}

// Possible actions that can be done with their stats
#[derive(Copy, Clone)]
pub enum Action {
    Damage(ActionStats<u32>),
    Heal(ActionStats<u32>),
    Weaken(ActionStats<f64>),
    Strengthen(ActionStats<f64>),
}

impl Action {
    // Returns the name of the action with a suffix
    pub fn name(&self) -> String {
        match self {
            Self::Damage(stats) => stats.name.to_string() + " (D)",
            Self::Heal(stats) => stats.name.to_string() + " (H)",
            Self::Weaken(stats) => stats.name.to_string() + " (W)",
            Self::Strengthen(stats) => stats.name.to_string() + " (S)",
        }
    }
}

// Name and amount of action
#[derive(Copy, Clone)]
pub struct ActionStats<T> {
    pub name: &'static str,
    pub value: T,
}

impl<T> ActionStats<T> {
    // Creates new stats for action
    pub const fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
}
