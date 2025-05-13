# lrxed

A tui application for synchronising lyrics

## Build deps

libasound.so=2-xx

## Goals

Initial release:

- [ ] import/export lrc sidecar files
- [ ] sync lyrics to current progress
- [ ] vim like keybinds
- [ ] adjust timestamp per line
- [ ] edit lyrics in text editor
- [ ] browse directory

Future:

- [ ] lrc tags integration
- [ ] lrclib integration
- [ ] enhanced lrc support (karaoke lyrics)
- [ ] match lrc files according to a naming scheme
- [ ] import/export id3 lyrics frames

Non-goals:

- Be a music player
- Built-in text editor

## Keybinds

Global:

- [x] q: exit lrxed from anywhere
- [ ] Enter: open/accept
- [ ] Esc: back/cancel

Lyrics editor view:

- [ ] Space: sync lyrics to current time and select next line
- [x] j: cursor down
- [x] k: cursor up
- [ ] h: cursor left
- [ ] l: cursor right
- [ ] J: append next line to current
- [ ] K: split line at cursor
- [ ] H: seek backwards
- [ ] L: seek forwards
- [x] \[0-9]: seek to \[0-9]0%
- [ ] r: \[r]esume/pause
- [ ] R: \[R]estart
- [x] g: \[g]o to first line
- [x] G: \[G]o to last line
- [ ] w: cursor to next \[w]ord
- [ ] e: cursor to \[e]nd of word
- [ ] b: cursor \[b]ackward by a word
- [ ] u: \[u]ndo
- [ ] ^r: \[^r]edo
- [ ] f: play \[f]rom here
- [ ] F: play \[F]rom here and pause at next timestamp
- [ ] t: go \[t]o currently playing line
- [ ] T: go \[T]o currently playing line + 1 down
- [ ] s: increment timestamp by a \[s]econd
- [ ] S: decrement timestamp by a \[S]econd
- [ ] d: increment timestamp by a \[d]ecisecond (0.10)
- [ ] D: decrement timestamp by a \[D]ecisecond (0.10)
- [ ] c: increment timestamp by a \[c]entisecond (0.01)
- [ ] C: decrement timestamp by a \[C]entisecond (0.01)
- [ ] z: set timestamp average of surrounding
- [ ] x: delete timestamp
- [ ] X: delete line
- [ ] y: \[y]ank timestamp
- [ ] Y: \[Y]ank line
- [ ] p: \[p]aste timestamp
- [ ] P: \[P]aste line
- [ ] i: edit line in \[i]nternal editor
- [ ] I: edit file in external editor
- [ ] o: insert new line after
- [ ] O: insert new line before
- [ ] a: toggle \[a]utoscroll
- [ ] A: toggle \[A]utoseek (play from newly selected line)
- [ ] [: volume down
- [ ] ]: volume up
- [ ] {: volume down
- [ ] }: volume up
- [ ] -: speed down
- [ ] +: speed up
- [ ] =: reset playback speed
