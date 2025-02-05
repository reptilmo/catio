# catio
A simple cat themed platforme game with basic physics.

## Features


## Requires SDL libraries installed on the system.
### Linux
Install these through your favourite package management tool, or via
http://www.libsdl.org/

Ubuntu example:
> sudo apt-get install libsdl2-dev

Fedora example:
> sudo dnf install SDL2-devel

Arch example:  
(Arch doesn't have separate regular and development packages, everything goes together.)  
> sudo pacman -S sdl2

You might also need a C compiler (`gcc`).

### macOS
#### Homebrew
On macOS, it's a good idea to install these via
[homebrew][homebrew].

```
brew install sdl2
```

In recent versions of Homebrew, the installed libraries are usually linked into `$(brew --prefix)/lib`.
If you are running an older version, the symlink for SDL might reside in `/usr/local/lib`.

To make linking libraries installed by Homebrew easier, do the following for your respective shell.

Add this line to your `~/.zshenv` or `~/.bash_profile` depending on whether you use ZSH or Bash.
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```
#### MacPorts
You can also get sdl2 via `macports`.

```
sudo port install libsdl2
```

Then add the following to your `~/.bash_profile` if not already present.
```
export LIBRARY_PATH="$LIBRARY_PATH:/opt/local/lib/"
```

If you're having issues with either Homebrew or MacPorts, [see here][pdev-issue].


