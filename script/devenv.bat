@ECHO OFF

REM Usage: ./devenv.sh <docker command> <image name> <image tag> <container name>
REM This program creates and logs in to docker image "theos-devenv" and container "theos-devenv".
REM This program should be called from "../Makefile" with command "make devenv".

SET ERROR=FALSE
IF "%1" == "" SET ERROR=TRUE
IF "%2" == "" SET ERROR=TRUE
IF "%3" == "" SET ERROR=TRUE
IF "%4" == "" SET ERROR=TRUE
IF %ERROR%==TRUE (
ECHO There should be 4 arguments.
EXIT /B -1
)

ECHO HELLO

