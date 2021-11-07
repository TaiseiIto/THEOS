@ECHO OFF

REM Usage: ./devenv.sh <docker command> <image name> <image tag> <container name>
REM This program creates and logs in to docker image "theos-devenv" and container "theos-devenv".
REM This program should be called from "../Makefile" with command "make devenv".

REM CHECK ARGS
SET ERROR=FALSE
IF "%1" == "" SET ERROR=TRUE
IF "%2" == "" SET ERROR=TRUE
IF "%3" == "" SET ERROR=TRUE
IF "%4" == "" SET ERROR=TRUE
IF %ERROR%==TRUE (
ECHO There should be 4 arguments.
EXIT /B -1
)

REM PICK ARGS
SET DOCKER=%1
SET IMAGE=%2
SET TAG=%3
SET CONTAINER=%4

REM MOVE TO DIRECTORY WHERE THIS SCRIPT IS PUT
SET CURRENTDIR=%~DP1
CALL :DIRNAME %0
CD %DIRNAME%

FOR /F "USEBACKQ DELIMS=" %%I IN ('%DOCKER% images') DO (
	ECHO %%I | FIND /I "%IMAGE%" > NUL
	IF ERRORLEVEL 1 %DOCKER% build --no-cache -t %IMAGE%:%TAG% ..
)
FOR /F "USEBACKQ DELIMS=" %%I IN ('%DOCKER% ps -a') DO (
	ECHO %%I | FIND /I "%CONTAINER%" > NUL
	IF ERRORLEVEL 1 ECHO There is not the container
)

CD %CURRENTDIR%

REM REPRODUCE UNIX DIRNAME COMMAND
:DIRNAME
SET DIRNAME=%~DP1
EXIT /B

