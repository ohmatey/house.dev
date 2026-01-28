# house.dev

your projects folder is full of ghosts.

node_modules from that thing you tried once. .git folders from abandoned experiments. rust targets that compiled a year ago.

house.dev finds them. shows you how much space they're hoarding. lets you yeet them into the trash.

## warning

this is experimental software built by someone who was tired of running `du -sh` over and over.

it will probably:
- work fine
- maybe not work fine
- definitely not work on linux yet

use at your own risk. it moves things to trash, not permanent delete, so you can fish stuff back out if needed.

## usage

```bash
pnpm install
pnpm tauri dev
```

pick a folder. watch it scan. delete the bloat.

## what it finds

node_modules, .git, target, build, dist, .next, vendor, __pycache__, .venv, Pods, DerivedData, and other usual suspects.

only shows folders > 1MB because who cares about the small ones.

## stack

tauri + rust + typescript + vibes

## license

MIT. do whatever.
