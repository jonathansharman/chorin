use std::{cell::RefCell, fmt::Display, rc::Rc};

use cursive::{
	direction::Orientation,
	event::Key,
	view::{Nameable, Resizable, Scrollable},
	views::{
		Dialog, EditView, LinearLayout, Panel, SelectView, SliderView, TextView,
	},
	View,
};
use rand::seq::SliceRandom;

#[derive(Debug)]
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

#[derive(Debug)]
struct Chore {
	title: String,
	cost: u32,
	priority: Priority,
}

struct State {
	name: String,
	due: Vec<Chore>,
}

type SharedState = Rc<RefCell<State>>;

fn main() {
	let due = vec![
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
	let state = Rc::new(RefCell::new(State {
		name: "Username".to_string(),
		due,
	}));

	let mut siv = cursive::default();
	siv.add_layer(get_main_menu(state.clone()));
	siv.add_global_callback(Key::Del, move |s| {
		s.call_on_name("due-select", |due_select: &mut SelectView<usize>| {
			if let Some(id) = due_select.selected_id() {
				state.borrow_mut().due.remove(id);
				due_select.remove_item(id);
			}
		});
		s.add_layer(Dialog::info(format!(
			"Now there are {} items: {:?}",
			state.borrow().due.len(),
			state.borrow().due,
		)));
	});
	siv.add_global_callback('q', |s| s.quit());
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

fn get_main_menu(state: SharedState) -> impl View {
	// Choose a title.
	let title = TITLE_FNS.choose(&mut rand::thread_rng()).unwrap()(
		&state.borrow().name,
	);
	// Add a SelectView for the currently due chores.
	let due_select = SelectView::new()
		.autojump()
		.with_all(state.borrow().due.iter().enumerate().map(|(idx, chore)| {
			let label = format!(
				"{}. {} ({}) [{}]",
				idx + 1,
				chore.title,
				chore.priority,
				chore.cost
			);
			(label, idx)
		}))
		.with_name("due-select")
		.scrollable();
	// Arrange everything vertically.
	Panel::new(
		LinearLayout::new(Orientation::Vertical)
			// General hotkeys
			.child(
				TextView::new("(e) edit chores (p) switch profile (q) quit")
					.fixed_height(2),
			)
			// Due chores
			.child(
				LinearLayout::new(Orientation::Vertical)
					.child(TextView::new(
						"(c) complete (o) obviate (del) abrogate",
					))
					.child(due_select),
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
