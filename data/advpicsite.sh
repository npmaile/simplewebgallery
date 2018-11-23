#!/usr/bin/bash
wd=$(pwd)
tempfile="$(mktemp)"

fd -a -L '.gif$|.png$|.jpg$|.jpeg$' | while read line 
do
	echo "\"/$line\"," >> $tempfile
done
cat $tempfile | sort -R
