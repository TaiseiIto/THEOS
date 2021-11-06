@ECHO OFF

REM Usage: $ clean_devenv.bat <docker command> <image name> <container name>
REM This program removes docker image "theos-devenv" and container "theos-devenv".
REM This program should be called from "../Makefile" with command "make clean-devenv".

SET ERROR=FALSE
IF "%1" == "" SET ERROR=TRUE
IF "%2" == "" SET ERROR=TRUE
IF "%3" == "" SET ERROR=TRUE
IF %ERROR%==TRUE (
ECHO There should be 3 arguments.
EXIT /B -1
)

ECHO Hello, World!

