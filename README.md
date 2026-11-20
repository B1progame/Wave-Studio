# Wave Studio

A browser music sketchpad inspired by the Pro Wave desktop interface, built with React, TypeScript and Vite. It includes a 16-step sequencer, five built-in sounds, transport and tempo controls, and JSON project save/load. It runs locally in the browser.

## Develop locally

Install Node.js 22 or newer, then run:

```sh
npm ci
npm run dev
```

Open the local URL printed by Vite. Run `npm run build` to check types and create `dist/`, and `npm run preview` to preview that build. GitHub Actions builds and deploys `dist/` to GitHub Pages on pushes to `main`.

This is an independent browser prototype, not a port of the Rust audio engine. Mixer strips are visual, and `.prowave` projects, recording, plug-ins, and the desktop app's editors are not supported. See [Pro Wave](https://github.com/fabox-os/Pro-Wave) for the original application. Distributed under GPL-3.0-or-later; see LICENSE.
