#!/usr/bin/env sh

start_time=00:11
duration=4

palette="/tmp/palette.png"

filters="fps=15,scale=320:-1:flags=lanczos"

# ffmpeg -v warning -ss $start_time -t $duration -i "$1" -vf "$filters,palettegen" -y $palette
# ffmpeg -v warning -ss $start_time -t $duration -i "$1" -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y "$2"

ffmpeg -ss $start_time -t $duration -i "$1" -filter_complex \
	"[0:v] fps=15,scale=320:-1,split [a][b];[a] palettegen [p];[b][p] paletteuse=new=1" -y "$2"
