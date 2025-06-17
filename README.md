# lrxed

[![Build](https://github.com/LunaPresent/lrxed/actions/workflows/build.yaml/badge.svg)](https://github.com/LunaPresent/lrxed/actions/workflows/build.yaml)

**/ˈlɪrɪksd/**  
A tui application for synchronising lyrics

![a showcase of the application](https://raw.githubusercontent.com/LunaPresent/readme-assets/refs/heads/lrxed/assets/showcase.gif)

## Installation

### Building from source

Dependencies:

- [libasound2-dev](https://launchpad.net/ubuntu/noble/+package/libasound2-dev) (Linux only)
- [rust toolchain](https://www.rust-lang.org/tools/install)

```sh
git clone https://github.com/LunaPresent/lrxed.git
cargo install --path ./lrxed --locked
```

### Cargo

```sh
cargo install lrxed
```

### Nix/NixOS

First add this repo as an input to your flake.

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    lrxed = {
      url = "github:LunaPresent/lrxed";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

#### Using the packages

```nix
{ inputs, pkgs, ... }:
{
  environment.systemPackages = [ inputs.lrxed.packages.${pkgs.stdenv.system.hostPlatform}.default ];
}
```

#### Using the overlays

```nix
{ inputs, ... }:
{
  nixpkgs.overlays = [ inputs.lrxed.overlays.default ];
  environment.systemPackages = [ pkgs.lrxed ];
}
```

#### Using the home-manager module

```nix
{ inputs, ... }:
{
  imports = [ inputs.lrxed.homeModules.default ];

  programs.lrxed = {
    enable = true;

    settings.keys.global.quit = [
      { key = "q"; }
      { key = "z"; }
    ];
  };
}
```

## Requirements

- A [nerd font](https://www.nerdfonts.com/), for displaying certain icons.
  This font must be set in your terminal emulator config.
- Set a default text editor, e.g. in the `$VISUAL` envvar on Linux, for the open-in-editor feature.
- A local music library. This may clash with your setup if you use a media server such as jellyfin.
  I suggest trying [sshfs](https://github.com/libfuse/sshfs) to mount the remote media, as running
  lrxed remotely will not give you good audio playback.

## Features & goals

- [x] play audio from .mp3, .flac, .wav and .ogg formats
- [x] import/export lrc sidecar files
- [x] sync lyrics to current progress
- [x] adjust timestamp per line
- [x] edit lyrics in text editor
- [x] browse directory
- [x] user config
- [ ] basic internal text editing
- [ ] media keys / playerctl support
- [ ] mouse controls
- [ ] lrc tags integration
- [ ] lrclib integration
- [ ] enhanced lrc support (karaoke lyrics)
- [ ] match lrc files according to a naming scheme
- [ ] import/export id3 lyrics frames
- [ ] sync with external music player (mpris)

Non-goals:

- Be a music player: Yes, it plays music, but lrxed does not aim to implement a queue or autoplay
  feature, or anything aimed specifically towards improving its viability as a music player.
- Be a text editor: While basic text editing entirely within lrxed is a (planned) feature, this will
  not stand up to any slightly advanced text editing features of \<insert favourite text editor>.
  For all your advanced text editing needs, you can use the open-in-editor feature (default \<I>)

## Configuration

You may choose between the toml, json or yaml file format to configure lrxed.

Print the path to the configuration file

```sh
lrxed --print-config-path [toml | json | yaml]
```

lrxed does not create this config file, you will need to create it manually.

To print your current configuration, or the defaults if not configured, use

```sh
lrxed --print-config [toml | json | yaml]
```

You can use this as a reference to make your configuration.  
The default value in these commands when file type is omitted is toml.

Alternatively, use the following oneliner to create the config and seed it with the defaults.  
Example for toml file type:

```sh
lrxed --print-config toml > (lrxed --print-config-path toml)
```

## Controls

| key                         | explanation                                      | config identifier      |
| --------------------------- | ------------------------------------------------ | ---------------------- |
| ?                           | show list of key bindings                        | view-keys              |
| q                           | \[q]uit lrxed                                    | quit                   |
| Enter                       | open/accept                                      | confirm                |
| Esc                         | back/cancel                                      | cancel                 |
| Space                       | sync lyrics to current time and select next line | sync-timestamp         |
| ^w                          | save                                             | save                   |
| j **or** Down               | cursor down                                      | move-cursor-y          |
| k **or** Up                 | cursor up                                        | move-cursor-y          |
| h **or** Left               | cursor left                                      | move-cursor-x          |
| l **or** Right              | cursor right                                     | move-cursor-x          |
| g                           | \[g]o to first line                              | set-cursor-y           |
| G                           | \[G]o to last line                               | set-cursor-y           |
| \_ **or** Home              | cursor to line start                             | set-cursor-x           |
| $ **or** End                | cursor to line end                               | set-cursor-x           |
| H                           | seek backwards                                   | seek-backwards         |
| L                           | seek forwards                                    | seek-forwards          |
| \[0-9]                      | seek to \[0-9]0%                                 | seek-relative          |
| r                           | \[r]esume/pause                                  | toggle-pause           |
| u                           | \[u]ndo                                          | undo                   |
| ^r                          | \[^r]edo                                         | redo                   |
| f                           | play \[f]rom selected word (line for now)        | seek-to-cursor         |
| F                           | play \[F]rom selected line                       | seek-to-cursor-line    |
| t                           | go \[t]o currently playing word (line for now)   | cursor-to-playing      |
| T                           | go \[T]o start of currently playing line         | cursor-to-playing-line |
| s                           | increment timestamp by a \[s]econd               | adjust-timestamp       |
| S                           | decrement timestamp by a \[S]econd               | adjust-timestamp       |
| d                           | increment timestamp by a \[d]ecisecond (0.10s)   | adjust-timestamp       |
| D                           | decrement timestamp by a \[D]ecisecond (0.10s)   | adjust-timestamp       |
| c                           | increment timestamp by a \[c]entisecond (0.01s)  | adjust-timestamp       |
| C                           | decrement timestamp by a \[C]entisecond (0.01s)  | adjust-timestamp       |
| I                           | edit file in external editor                     | open-in-editor         |
| [                           | volume down                                      | change-volume          |
| ]                           | volume up                                        | change-volume          |
| {                           | volume down slightly                             | change-volume          |
| }                           | volume up slightly                               | change-volume          |
| -                           | speed down                                       | change-speed           |
| +                           | speed up                                         | change-speed           |
| =                           | reset playback speed                             | reset-speed            |
| h **or** Left **or** Esc    | leave directory                                  | leave-directory        |
| l **or** Right **or** Enter | go into directory or edit file                   | open-file-or-directory |

### To be done

| key | explanation                                        |
| --- | -------------------------------------------------- |
| J   | append next line to current                        |
| K   | split line at cursor                               |
| w   | cursor to next \[w]ord                             |
| e   | cursor to \[e]nd of word                           |
| b   | cursor \[b]ackward by a word                       |
| ^s  | set timestamp average of \[s]urrounding            |
| x   | delete timestamp                                   |
| X   | delete line                                        |
| y   | \[y]ank timestamp                                  |
| Y   | \[Y]ank line                                       |
| p   | \[p]aste timestamp                                 |
| P   | \[P]aste line                                      |
| i   | edit line in \[i]nternal editor                    |
| o   | insert new line after                              |
| O   | insert new line before                             |
| a   | toggle \[a]utoscroll                               |
| A   | toggle \[A]utoseek (play from newly selected line) |

## Related projects

Since you're probably here because you maintain your own media library (with lyrics),
I highly recommend you check out the following software if you haven't already:

- [**lrcget**](https://github.com/tranxuanthang/lrcget) - Does everything lrxed does and more,
  but in a gui and with more traditional, text-input-first controls.
  Definitely use this for obtaining .lrc files from the internet before editing them,
  at least until lrxed implements lrclib integration :3

- [**sptlrx**](https://github.com/raitonoberu/sptlrx) - Simple looking but amazing terminal
  application to use alongside your favourite music player and get a nice synchronised lyrics viewer.

- [**cmus**](https://github.com/cmus/cmus) - This is just a really solid music player,
  I use this alongside sptlrx. Doesn't support lyrics out of the box, but plugins may exist.

- [**jellyfin-tui**](https://github.com/dhonus/jellyfin-tui) - If you have a jellyfin server, are
  addicted to your terminal and also you want a nice built-in synchronised lyrics viewer, look no
  further. Works out of the box with .lrc files as long as the jellyfin server can serve them.

- [**harmonoid**](https://github.com/harmonoid/harmonoid) - Very pretty GUI music player with
  .lrc compatible scrolling lyrics viewer. Warning: this is freeware but not open source.

- [**termusic**](https://github.com/tramhao/termusic) - Music player built in rust. Has support for
  synced lyrics but I couldn't get it to recognise .lrc files.

---

Thank you for checking out my project. Please consider leaving a star if you like it, or if you
just feel like giving me a confidence boost.

Making issues and pull requests is very much appreciated. Please be patient with me as this is my
first experience maintaining an open source project. Any tips or general feedback are welcome in the
discussion tab.
