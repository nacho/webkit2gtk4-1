# webkit2gtk4-1-rs

The Rust bindings of [webkit2gtk 4.1](https://webkitgtk.org/)

## Documentation

This project targets bindings for WebkitGTK 4.1. If your intention is to use
a different version of WebkitGTK then there are other projects:

 - WebkitGTK 4.0: https://github.com/nacho/webkit2gtk4-0-rs
 - WebkitGTK 5.0: https://gitlab.gnome.org/World/Rust/webkit2gtk5-rs

## Updating Webkit gir files

You can update all the files by doing:

```console
$ cd webkit2gtk-gir-files
$ flatpak remote-add --user --if-not-exists gnome-nightly https://nightly.gnome.org/gnome-nightly.flatpakrepo
$ flatpak install org.gnome.Sdk//master -y --noninteractive
$ flatpak run --command=python3 --filesystem=home org.gnome.Sdk//master dl.py
$ ./reformat.sh && ./fix.sh
```
