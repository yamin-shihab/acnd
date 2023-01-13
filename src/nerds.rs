use console_engine::Color;

// Array of nerds (last is secret)
pub const NERDS: [&Nerd; 5] = [&JOE, &ISAAC, &WILLIAM, &SUZIE, &YAMIN];

// Color of selected nerd in game and menu
pub const CURRENT_NERD_COLOR: Color = Color::Green;
pub const WAITING_NERD_COLOR: Color = Color::Red;

// Values that change damage done
const BASE_MULTIPLIER: i32 = 10;
const CRITICAL_CHANCE: i32 = 20;
const CRITICAL_MULTIPLIER: i32 = 2;

// Used to represent the two players
pub type Nerds = [Nerd; 2];

// Balanced nerd
const JOE: Nerd = Nerd::new(
    "Joe",
    200,
    [
        Action::new("Slap", ActionType::Damage, 3),
        Action::new("Band-Aid", ActionType::Heal, 2),
        Action::new("Pinch", ActionType::Weaken, 2),
        Action::new("Khan Academy", ActionType::Strengthen, 2),
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
const ISAAC: Nerd = Nerd::new(
    "Isaac",
    100,
    [
        Action::new("Meter Ruler Katana", ActionType::Damage, 6),
        Action::new("Self Confidence/Motivation", ActionType::Heal, 2),
        Action::new("Threaten with Scissors", ActionType::Weaken, 3),
        Action::new("Steroids", ActionType::Strengthen, 1),
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
const WILLIAM: Nerd = Nerd::new(
    "William",
    400,
    [
        Action::new("Curse/Swear Words", ActionType::Damage, 3),
        Action::new("Meditation", ActionType::Heal, 1),
        Action::new("Intimidating Stare", ActionType::Weaken, 1),
        Action::new("Inflatable Dumbbells", ActionType::Strengthen, 3),
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
const SUZIE: Nerd = Nerd::new(
    "Suzie",
    200,
    [
        Action::new("Insult", ActionType::Damage, 1),
        Action::new("First Aid Kit", ActionType::Heal, 4),
        Action::new("Threaten to Tell Teacher", ActionType::Weaken, 1),
        Action::new("Watch Dhar Mann Video", ActionType::Strengthen, 3),
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

// Overpowered nerd
const YAMIN: Nerd = Nerd::new(
    "Yamin",
    1000000,
    [
        Action::new("Do literally nothing", ActionType::Damage, 100),
        Action::new("Do pretty much nothing", ActionType::Heal, 100),
        Action::new("Do basically nothing", ActionType::Weaken, 100),
        Action::new("Do figuratively nothing", ActionType::Strengthen, 100),
    ],
    " ____
/    \\
\\ .. /
 \\<>/
 /||\\
/ || \\
 /  \\
^    ^",
);

// A character/player with their stats
#[derive(Copy, Clone)]
pub struct Nerd {
    pub name: &'static str,
    pub health: i32,
    pub multiplier: i32,
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

    // Returns the equation to be answered and the answer to it
    pub fn equation(&self, action: usize, nerd: &Nerd, critical: bool) -> (String, i32) {
        let critical = if critical { CRITICAL_MULTIPLIER } else { 1 };
        let action = self.actions[action];
        match action.action_type {
            ActionType::Damage => (
                format!(
                    "{} - {} * {} * {}",
                    nerd.health, action.value, self.multiplier, critical
                ),
                nerd.health - action.value * self.multiplier * critical,
            ),
            ActionType::Heal => (
                format!(
                    "{} + {} * {} * {}",
                    self.health, action.value, self.multiplier, critical
                ),
                self.health + action.value * self.multiplier * critical,
            ),
            ActionType::Weaken => (
                format!("{} - {} * {}", nerd.multiplier, action.value, critical),
                nerd.multiplier - action.value * critical,
            ),
            ActionType::Strengthen => (
                format!("{} + {} * {}", self.multiplier, action.value, critical),
                self.multiplier + action.value * critical,
            ),
        }
    }

    // Returns a critical hit multiplier
    pub fn critical() -> bool {
        fastrand::i32(0..100) < CRITICAL_CHANCE
    }

    // Uses the given action index
    pub fn use_action(
        &mut self,
        action: usize,
        value: i32,
        critical: bool,
        nerd: &mut Nerd,
    ) -> String {
        match self.actions[action].action_type {
            ActionType::Damage => nerd.health = value,
            ActionType::Heal => self.health = value,
            ActionType::Weaken => nerd.multiplier = value,
            ActionType::Strengthen => self.multiplier = value,
        }
        self.action_message(action, critical, nerd)
    }

    // Returns a message to be displayed as a result of an action
    fn action_message(&self, action: usize, critical: bool, nerd: &Nerd) -> String {
        format!(
            "{} used {} against {}{}",
            self.name,
            self.actions[action].name(),
            nerd.name,
            if critical
                && (self.actions[action].action_type == ActionType::Weaken
                    || self.actions[action].action_type == ActionType::Strengthen)
            {
                " (CRITICAL!!!)"
            } else {
                ""
            }
        )
    }
}

// Name and amount of action
#[derive(Copy, Clone)]
pub struct Action {
    name: &'static str,
    action_type: ActionType,
    value: i32,
}

impl Action {
    // Creates new stats for action
    pub const fn new(name: &'static str, action_type: ActionType, value: i32) -> Self {
        Self {
            name,
            action_type,
            value,
        }
    }

    // Returns the name of the action with a suffix
    pub fn name(&self) -> String {
        match self.action_type {
            ActionType::Damage => format!("{} ({}d)", self.name, self.value),
            ActionType::Heal => format!("{} ({}h)", self.name, self.value),
            ActionType::Weaken => format!("{} ({}w)", self.name, self.value),
            ActionType::Strengthen => format!("{} ({}s)", self.name, self.value),
        }
    }
}

// Possible actions that can be done with their stats
#[derive(Copy, Clone, PartialEq)]
pub enum ActionType {
    Damage,
    Heal,
    Weaken,
    Strengthen,
}
