// Values that change damage done
const BASE_MULTIPLIER: f64 = 1.0;
const CRITICAL_CHANCE: u32 = 15;
const CRITICAL_MULTIPLIER: u32 = 2;

// Balanced nerd
pub const JOE: Nerd = Nerd::new(
	"Joe",
	200,
	10,
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
	5,
	[
		Action::Damage(ActionStats::new("Meter Ruler Katana", 60)),
		Action::Heal(ActionStats::new("Self Confidence and Motivation", 20)),
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
	20,
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
	10,
	[
		Action::Damage(ActionStats::new("Insult", 15)),
		Action::Heal(ActionStats::new("First Aid Kit", 40)),
		Action::Weaken(ActionStats::new("Threaten to Tell Teacher", 0.10)),
		Action::Strengthen(ActionStats::new("Watch Dhar Man Video", 0.40)),
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

// Name and amount of action
#[derive(Copy, Clone)]
pub struct ActionStats<T> {
	name: &'static str,
	value: T,
}

impl<T> ActionStats<T> {
	// Creates new stats for action
	pub const fn new(name: &'static str, value: T) -> Self {
		Self { name, value }
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

// A character/player with their stats
#[derive(Copy, Clone)]
pub struct Nerd {
	pub name: &'static str,
	health: i32,
	defense: i32,
	multiplier: f64,
	actions: [Action; 4],
	pub sprite: &'static str,
}

impl Nerd {
	// Creates a new nerd
	pub const fn new(
		name: &'static str,
		health: i32,
		defense: i32,
		actions: [Action; 4],
		sprite: &'static str,
	) -> Self {
		Self {
			name,
			health,
			defense,
			multiplier: BASE_MULTIPLIER,
			actions,
			sprite,
		}
	}
}
