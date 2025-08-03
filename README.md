# Laurel

[![Rust](https://github.com/MadenedDM/laurel/actions/workflows/rust.yml/badge.svg)](https://github.com/MadenedDM/laurel/actions/workflows/rust.yml)
![Static Badge](https://img.shields.io/badge/License-MIT-fc0a9b)
<!-- [![Coverage Status](https://coveralls.io/repos/github/MadenedDM/laurel/badge.svg?branch=master)](https://coveralls.io/github/MadenedDM/laurel?branch=master) -->

This project is VERY WIP, don't expect it to run, work, be safe, or be user friendly. 

Don't worry about the following that much.

Default assets can be obtained by calling `get_assets.sh`, this file probably won't work on windows. Files in the assets/game path should be 16x16 pngs and files in the assets/ui path should be 8x16 pngs.

# Design Choices/Goals

1. Compatability with at least Linux and Windows, Mac would be nice but is not at this point going to be maintained directly.
2. Decoupling of Server and Client, the Server should rarely require features provided in the client, and these should only be done at runtime. This goal may be dropped at a later date if it becomes to tedious to uphold.

# Dream Features
I want to / plan to add these features but my own laziness and stupidity stands in the way:
- Match the contents of [Yet Another Roguelike Tutorial - Written in Python 3 and TCOD](https://rogueliketutorials.com/tutorials/tcod/v2/) except in the crazy design of this
- This list
- The actual game
