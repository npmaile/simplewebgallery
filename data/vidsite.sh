#!/usr/bin/bash
wd=$(pwd)
tempfile="$(mktemp)"

fd -L '.webm$|.mp4$' | while read line 
do
	echo "\"/$line\"," >> $tempfile
done
cat $tempfile | sort -R
