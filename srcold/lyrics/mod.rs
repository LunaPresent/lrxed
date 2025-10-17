pub mod editing;
mod lyric_line;
mod lyrics;
mod metadata;
mod time_index;
mod timestamp;

pub use lyric_line::LyricLine;
pub use lyrics::Lyrics;
pub use time_index::{TimeIndex, TimeIndexEntry, TimeIndexHint};
pub use timestamp::Timestamp;
