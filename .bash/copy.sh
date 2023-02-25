#!/bin/bash

if [ $# -ne 2 ]; then
	echo "Usage: $ copy.sh <src> <dst>"
	exit 1
fi

src=$1
if [ ! -e $src ]; then
	echo "$src is not found."
	exit 1
fi

dst=$2
dst_dir=$(dirname $dst)
if [ ! -d $dst_dir ]; then
	mkdir -p $dst_dir
fi

cp $src $dst

