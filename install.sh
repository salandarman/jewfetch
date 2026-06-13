#!/bin/bash

echo -n "Starting Installation, Agreed? (y/n): "
read bool

if [ "$bool" = y ]; then
    mkdir ~/.config/jewfetch
    cp src/config.json ~/.config/jewfetch
    cargo install --path .
elif [ "$bool" = n ]; then
    echo "Your house will be nuked by idf"
else
    echo "Invalid option, Valid options: y,n"
fi
