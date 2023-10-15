@ECHO OFF
CLS

copy src-tauri\windows\resources\*.dll src-tauri\
cargo tauri build --debug
del src-tauri\*.dll