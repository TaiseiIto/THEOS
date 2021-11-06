#!/bin/sh

# This program removes docker image "theos-devenv" and container "theos-devenv".
# This program should be called from "../Makefile" with command "make remove-devenv".

if [ $# -eq 3 ]; then
	docker=$1
	image=$2
	container=$3
	if [ -n "$($docker ps -a | grep $container)" ]; then
		$docker rm $container
	fi
	if [ -n "$($docker ps -a | grep $image)" ]; then
		$docker rmi $image
	fi
else
	echo There should be 3 arguments
fi

