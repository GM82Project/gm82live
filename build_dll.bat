cargo build --target i686-pc-windows-msvc --release

del gm82live.dll
move .\target\i686-pc-windows-msvc\release\gm82live.dll gm82live.dll

upx --lzma gm82live.dll

pause