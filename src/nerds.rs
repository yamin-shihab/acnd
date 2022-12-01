const BASE_MULTIPLIER: f64 = 1.0;
const CRITICAL_CHANCE: u32 = 15;
const CRITICAL_MULTIPLIER: u32 = 2;

pub const JOE: Nerd = Nerd::new(
	"Joe",
	200,
	10,
	[
		Action::Damage {
			name: "Slap",
			value: 30,
		},
		Action::Heal {
			name: "Band-Aid",
			value: 20,
		},
		Action::Weaken {
			name: "Pinch",
			value: 0.20,
		},
		Action::Strengthen {
			name: "Khan Academy",
			value: 0.20,
		},
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

pub const ISAAC: Nerd = Nerd::new(
	"Isaac",
	100,
	5,
	[
		Action::Damage {
			name: "Meter Ruler Katana",
			value: 60,
		},
		Action::Heal {
			name: "Self Confidence and Motivation",
			value: 20,
		},
		Action::Weaken {
			name: "Threaten with Scissors",
			value: 0.40,
		},
		Action::Strengthen {
			name: "Steroids",
			value: 0.20,
		},
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

pub const WILLIAM: Nerd = Nerd::new(
	"William",
	400,
	20,
	[
		Action::Damage {
			name: "Curse/Swear Words",
			value: 30,
		},
		Action::Heal {
			name: "Meditation",
			value: 10,
		},
		Action::Weaken {
			name: "Intimidating Stare",
			value: 0.10,
		},
		Action::Strengthen {
			name: "Inflatable Dumbbells",
			value: 0.20,
		},
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

pub const SUZIE: Nerd = Nerd::new(
	"Suzie",
	200,
	10,
	[
		Action::Damage {
			name: "Insult",
			value: 15,
		},
		Action::Heal {
			name: "First Aid Kit + 911",
			value: 40,
		},
		Action::Weaken {
			name: "Threaten to Tell Teacher",
			value: 0.10,
		},
		Action::Strengthen {
			name: "Watch Dhar Man Video",
			value: 0.40,
		},
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

pub struct Nerd {
	name: &'static str,
	health: i32,
	defense: i32,
	multiplier: f64,
	actions: [Action; 4],
	pub sprite: &'static str,
}

impl Nerd {
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

pub enum Action {
	Damage { name: &'static str, value: u32 },
	Heal { name: &'static str, value: u32 },
	Weaken { name: &'static str, value: f64 },
	Strengthen { name: &'static str, value: f64 },
}
