#!/bin/bash

if [ -x "$(command -v apt-get)" ]; then sudo apt-get install r-base cargo
elif [ -x "$(command -v dnf)" ];     then sudo dnf install R cargo
elif [ -x "$(command -v zypper)" ];  then sudo zypper install R-base-devel cargo
elif [ -x "$(command -v pacman)" ]; then sudo pacman -S r cargo
else echo "Err: Package manager not found... You must manually install: R, and the package cargo">&2; fi