@echo off

setlocal enabledelayedexpansion

set base_name="lc"
set count=1

:loop
if exist "%base_name%%count%" (
    set /a count+=1
    goto loop
) else (    
    echo Creating Rust project: %base_name%%count%
    cargo new %base_name%%count%
)

endlocal