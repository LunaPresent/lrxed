#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmBoxAction {
	#[default]
	Yes,
	No,
	Cancel,
}

impl ConfirmBoxAction {
	pub fn next(self) -> Self {
		match self {
			ConfirmBoxAction::Yes => ConfirmBoxAction::No,
			ConfirmBoxAction::No => ConfirmBoxAction::Cancel,
			ConfirmBoxAction::Cancel => ConfirmBoxAction::Cancel,
		}
	}

	pub fn prev(self) -> Self {
		match self {
			ConfirmBoxAction::Yes => ConfirmBoxAction::Yes,
			ConfirmBoxAction::No => ConfirmBoxAction::Yes,
			ConfirmBoxAction::Cancel => ConfirmBoxAction::No,
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModalState {
	pub confirm_box_selected: ConfirmBoxAction,
}
