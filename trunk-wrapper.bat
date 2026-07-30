@echo off
setlocal
set "args="
:loop
if "%~1"=="" goto end
if /i "%~1"=="--no-color" goto skiparg
if /i "%~1"=="--no-color=1" goto skiparg
set "args=%args% %1"
:skiparg
shift
goto loop
:end
if defined args (
    trunk %args%
) else (
    trunk
)
endlocal
