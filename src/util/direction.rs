pub trait Direction: core::fmt::Debug + Clone + Copy + PartialEq + Eq {
	fn x(self) -> i16;
	fn y(self) -> i16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadDirection {
	Up,
	Left,
	Down,
	Right,
}

impl Direction for QuadDirection {
	fn x(self) -> i16 {
		match self {
			Self::Left => -1,
			Self::Right => 1,
			_ => 0,
		}
	}

	fn y(self) -> i16 {
		match self {
			Self::Up => -1,
			Self::Down => 1,
			_ => 0,
		}
	}
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OctDirection {
	Up,
	UpLeft,
	Left,
	DownLeft,
	Down,
	DownRight,
	Right,
	UpRight,
}

impl Direction for OctDirection {
	fn x(self) -> i16 {
		match self {
			Self::UpLeft => -1,
			Self::Left => -1,
			Self::DownLeft => -1,
			Self::DownRight => 1,
			Self::Right => 1,
			Self::UpRight => 1,
			_ => 0,
		}
	}

	fn y(self) -> i16 {
		match self {
			Self::Up => -1,
			Self::UpLeft => -1,
			Self::DownLeft => 1,
			Self::Down => 1,
			Self::DownRight => 1,
			Self::UpRight => -1,
			_ => 0,
		}
	}
}
