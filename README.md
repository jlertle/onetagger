# ![Logo](https://raw.githubusercontent.com/Marekkon5/onetagger/master/assets/32x32.png) OneTagger

Cross-platform music tagger written in Rust, Vue.js and Quasar.
It can fetch metadata from Beatport, Traxsource, Juno Download and Discogs.
It is also able to fetch Spotify's Audio Features based on ISRC & exact match. 
There is a manual tag editor and quick tag editor which lets you use keyboard shortcuts.

MP3, AIFF, FLAC, M4A (AAC, ALAC) supported.

For more info check out our [website](https://onetagger.github.io/).

![Screenshot](https://raw.githubusercontent.com/Marekkon5/onetagger/master/assets/screenshot.jpg)

## Installing

You can download latest binaries from [releases](https://github.com/Marekkon5/onetagger/releases)

### Linux

You might need to install additional dependencies to make One Tagger work:
```
sudo apt install libsndfile1-dev
```

## Credits
Bas Curtiz - UI, Idea, Help

## Support
You can support this project by donating on [PayPal](https://paypal.me/marekkon5) or [Patreon](https://www.patreon.com/onetagger)

## Compilling

### Linux & Mac
Mac version is cross compiled from linux, the build script is designed to run on Github Actions enviromnent, so you have to install some dependencies manually: [rustup](https://rustup.rs), [node](https://nodejs.org/en/download/package-manager/)

**Install remaining dependencies**
```
sudo apt install -y autogen libsndfile1-dev libasound2-dev pkg-config make libssl-dev gcc g++ curl wget git libwebkit2gtk-4.0-dev
```

**Compile Linux only**
```
cargo build --release
```
Output is in: `target/release/onetagger`

**Compile Linux + Mac, create bundles**
```
assets/compile-nix.sh
```
**Don't start the script directly or from assets dir. It has to be relative to the parent**  
Output files will be in the `dist/` directory.

### Windows
You need to install dependencies: [rustup](https://rustup.rs), [node](https://nodejs.org/en/download/), [Visual Studio 2019 Build Tools](https://visualstudio.microsoft.com/downloads/)

**Compile UI:**
```
cd client
npm i
npm run build
cd ..
```

**Compile OneTagger:**
```
copy assets\sndfile.lib .
cargo build --release
```

Output will be inside `target\release` folder.