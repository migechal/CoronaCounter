#!/bin/bash

packagesNeeded='r cargo'
if [ -x "$(command -v apk)" ];       then sudo apk add --no-cache $packagesNeeded
elif [ -x "$(command -v apt-get)" ]; then sudo apt-get install $packagesNeeded
elif [ -x "$(command -v dnf)" ];     then sudo dnf install $packagesNeeded
elif [ -x "$(command -v zypper)" ];  then sudo zypper install $packagesNeeded
elif [ -x "$(command -v pacman)" ]; then sudo pacman -S $packagesNeeded
else echo "Err: Package manager not found... You must manually install: $packagesNeeded">&2; fi