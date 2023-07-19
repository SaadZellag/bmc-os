# bmc-os

This project is a custom OS that followed the structure from [Blog OS](https://github.com/phil-opp/blog_os). It is an OS to play the game of chess against my own custom engine.

## Features:
- Full chess game playable
- 2200-ish computer rated engine to play against (rating may change due to the change in environment)
- Engine eval when playing against it
- Engine difficulty selector


# How to run it
Currently there is a `./run_os.sh` script that builds the project, runs it and launches VNC viewer

Another way is to run `cargo run --release` then open up an alternative to VNC viewer on port 5900