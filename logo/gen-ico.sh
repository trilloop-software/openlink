#!/bin/bash

set -ex

svg=$1

size=(16 32 128 256)

out="temp"

echo Making bitmaps from your svg...

for i in ${size[@]}; do
  inkscape $svg -w$i -h$i
done

echo Compressing...

## Replace with your favorite (e.g. pngquant)
# optipng -o7 "$out/*.png"
pngquant -f --ext .png "$out/*.png" --posterize 4 --speed 1

echo Converting to favicon.ico...

convert "$out/*.png" favicon.ico

# Clean-up
rm -rf "$out/"

echo Done