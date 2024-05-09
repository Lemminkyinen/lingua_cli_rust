#!/bin/bash

for file in files/tones/*.mp3
do
    newfile=$(basename "$file" _MP3.mp3).mp3
    ffmpeg -i "$file" -map_metadata -1 -b:a 112k "files/tones2/$newfile" -y
done