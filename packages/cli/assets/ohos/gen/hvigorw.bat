@echo off
REM Hvigor wrapper script for Windows
REM This script is used to invoke the hvigor build system

setlocal

set HVIGOR_WRAPPER_VER=4.0.2

REM Resolve links: %~dp0 may be a link
set APP_HOME=%~dp0

REM Check if node is available
where node >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Node.js is not installed or not in PATH
    echo Please install Node.js to use hvigor
    exit /b 1
)

REM Check if hvigor exists locally
set HVIGOR_CMD=
if exist "%APP_HOME%hvigor\hvigor-wrapper.js" (
    set HVIGOR_CMD=%APP_HOME%hvigor\hvigor-wrapper.js
) else if exist "%APP_HOME%node_modules\@ohos\hvigor\bin\hvigor.js" (
    set HVIGOR_CMD=%APP_HOME%node_modules\@ohos\hvigor\bin\hvigor.js
) else (
    set HVIGOR_CMD=hvigor
)

REM Execute hvigor with all arguments passed to this script
if "%HVIGOR_CMD%"=="hvigor" (
    node %HVIGOR_CMD% %*
) else (
    node "%HVIGOR_CMD%" %*
)
