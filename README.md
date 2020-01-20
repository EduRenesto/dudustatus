# dudustatus

This is a quick Rust project that I hacked together to get a nice and beautiful
output to feed into `lemonbar`.

# Using

Clone this repository, and edit your `src/main.rs`, selecting your preferred
lemonbar settings and your modules. Note that the colors for the modules are 
currently hardcoded into them, so you'll need to change the `src/modules` folder
to rice things a bit more.

# Current Modules

I wrote a few simple modules, that should work as a base to others. The modules are:

- **bspwm**: shows info about the current desktops
- **cpu**: shows the current cpu usage using `ps`
- **pamixer**: shows pulseaudio volume
- **ram**: shows RAM usage using `ps`
- **spacer**: simple spacer
- **time**: shows date and time info
- **weather**: fetches weather info from OpenWeatherMap. If you want it to work,
 you need to set the `OWP_APP_KEY` environment variable before starting `dudustatus`.

# Roadmap

This will grow and will become way more configurable. Initially I wanted it to be
a bit `suckless`, with config being hardcoded into the code, but talking to friends
made me reconsider using dedicated config files. I've not made up my mind, tho.

# Using

You'll need the following stuff to run the code as it is right now:

- lemonbar-xft
- a working Rust toolchain
- the following fonts:
    - Overpass
    - icomoon-feather (I got this one from the polybar-themes repository)

Once you're done with the dependencies, a nice `cargo run` will already work. If you 
want to autostart it with your VM, I'd reccomend running a `cargo build --release` and
copying the resulting `target/release/dudustatus` to somewhere you can run.

# Contributing

Feel free to contribute anything you'd like, from core refactors to more modules. 

# License

For now this is licensed under the WTFPL.
