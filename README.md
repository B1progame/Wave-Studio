# Wave Studio

A browser music sketchpad inspired by [Pro Wave](https://github.com/fabox-os/Pro-Wave). This is an independent prototype, not a port of the Rust DAW. Its sequencer, synthesized sounds, tempo, volume, and JSON project files work locally in a modern browser. It does not open `.prowave` files or provide Pro Wave's audio recording, mixer, plug-ins, or editors.

## Run locally

From this directory run `python3 -m http.server 8000`, then open `http://localhost:8000/`. On a phone connected to the same network, listen on an appropriate interface and use the computer's local address. Browser support and audio output vary by device. Audio begins only after pressing Play.

## GitHub Pages

Publish the contents of this `web` directory as a static site (for example, copy them to the root of a new public repository and enable Pages from its default branch). All links to site assets are relative, so it works under a project URL such as `https://USERNAME.github.io/REPOSITORY/`.

The original Pro Wave source is GPL-3.0-or-later; see the repository's LICENSE. This prototype is distributed under the same terms.
