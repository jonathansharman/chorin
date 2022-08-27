use std::fmt::Display;

use cursive::{
	direction::Orientation,
	view::{Nameable, Resizable, Scrollable},
	views::{
		Dialog, EditView, LinearLayout, Panel, SelectView, SliderView, TextView,
	},
	View,
};
use rand::seq::SliceRandom;

enum Priority {
	Low,
	Mid,
	High,
}

impl Display for Priority {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Priority::Low => f.write_str("low"),
			Priority::Mid => f.write_str("mid"),
			Priority::High => f.write_str("high"),
		}
	}
}

struct Chore {
	title: String,
	cost: u32,
	priority: Priority,
}

fn main() {
	let mut siv = cursive::default();

	let name = "Username";
	let chores_due = vec![
		Chore {
			title: "Chore 1".to_string(),
			cost: 5,
			priority: Priority::High,
		},
		Chore {
			title: "Chore 2".to_string(),
			cost: 3,
			priority: Priority::Mid,
		},
		Chore {
			title: "Chore 3".to_string(),
			cost: 1,
			priority: Priority::Low,
		},
	];

	siv.add_layer(get_main_menu(&chores_due, name));

	// s.add_layer(
	// 	Dialog::text(text)
	// 		.title("Selection")
	// 		.button("Quit", |s| s.quit()),
	// );

	siv.run();
}

const TITLE_FNS: &[fn(&str) -> String] = &[
	|name| format!("Pitter patter, let's get at 'er, {}.", name),
	|name| format!("{}, how're ya now?", name),
	|name| format!("It's a great day for hay, {}.", name),
	|name| format!("You'd best be preparin' for a donnybrook, {}.", name),
	|name| format!("Let's take about 5 to 10% off 'er over there, {}.", name),
	|name| format!("Back to chorin', {}.", name),
];

fn get_main_menu(chores: &[Chore], name: &str) -> impl View {
	// Choose a title.
	let title = TITLE_FNS.choose(&mut rand::thread_rng()).unwrap()(name);
	// Add a SelectView for the currently due chores.
	let chore_select = SelectView::new()
		.autojump()
		.with_all(chores.iter().enumerate().map(|(idx, chore)| {
			let label = format!(
				"{}. {} ({}) [{}]",
				idx + 1,
				chore.title,
				chore.priority,
				chore.cost
			);
			(label, idx)
		}))
		.with_name("chore-select")
		.scrollable();
	// Arrange everything vertically.
	Panel::new(
		LinearLayout::new(Orientation::Vertical)
			// General hotkeys
			.child(
				TextView::new("(e) edit chores (p) switch profile")
					.fixed_height(2),
			)
			// Due tasks
			.child(
				LinearLayout::new(Orientation::Vertical)
					.child(TextView::new(
						"(c) complete (o) obviate (del) abrogate",
					))
					.child(chore_select),
			),
	)
	.title(title)
}

fn get_add_chore(chores: &[Chore]) -> SelectView<usize> {
	let mut chore_select = SelectView::new().autojump();
	chore_select.set_on_submit(|s, idx| {
		// Title
		let title_edit = EditView::new().on_submit(|s, text| {
			s.pop_layer();
		});
		s.add_layer(Dialog::around(title_edit).title("Name the chore"));
		// Priority
		let priority_select = SelectView::new()
			.item("Low", Priority::Low)
			.item("Medium", Priority::Mid)
			.item("High", Priority::High)
			.on_submit(|s, priority| {
				s.pop_layer();
			});
		s.add_layer(
			Dialog::around(priority_select)
				.title("How high is the priority for this?"),
		);
		// Cost
		let cost_slider = SliderView::horizontal(10).on_enter(|s, cost| {
			s.pop_layer();
		});
		s.add_layer(
			Dialog::around(cost_slider)
				.title("How many spoons will this cost?"),
		);
	});
	for (idx, chore) in chores.iter().enumerate() {
		chore_select.add_item(chore.title.clone(), idx);
	}
	chore_select
}
