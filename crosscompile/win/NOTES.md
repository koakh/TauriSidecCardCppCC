changes in cc.cpp for work in windows

https://stackoverflow.com/questions/158585/how-do-you-add-a-timed-delay-to-a-c-program
	https://stackoverflow.com/a/158614

changed
cout << "Caught exception in some SDK method. Error: " << e.GetMessage() << endl;
to
cout << "Caught exception in some SDK method. Error: " << e.GetError() << endl;




install WSL
wsl --install




https://stackoverflow.com/questions/69332534/how-to-pack-more-dlls-into-tauri-bundle
https://wixtoolset.org/docs/schema/wxs/registryvalue/#:~:text=string%3A%20The%20value%20is%20interpreted%20and%20stored%20as,interpreted%20and%20stored%20as%20an%20expandable%20string%20%28REG_EXPAND_SZ%29.



rustc -Vv | grep host | cut -f2 -d' '
On Windows you can use PowerShell instead:
rustc -Vv | Select-String "host:" | ForEach-Object {$_.Line.split(" ")[1]}
x86_64-pc-windows-msvc


 pnpm cargo tauri build --debug


https://stackoverflow.com/questions/69332534/how-to-pack-more-dlls-into-tauri-bundle



scan registry
https://stackoverflow.com/a/13781995
https://www.nirsoft.net/utils/reg_file_from_application.html
https://launcher.nirsoft.net/downloads/index.html ---> password nirsoft9876$
https://sourceforge.net/projects/regshot/

snapshot1
C:\Users\mario\Downloads\nirsoft_package_enc_1.30.6\NirSoft\RegSnapshot20231013234425

Use 7Zip.
It will unpack MSI , CAB, some EXE, and a lot more packages for you .There is no need for admin privileges to run this either.
And its open source, so no nagging messages begging you to buy the stuff

fezada parece q os certificados so precisam de estar nesta folder e mais nada
"c:\Program Files\Portugal Identity Card\eidstore" 
com a app instalada, e apagando tudo

mas ha qql coisa ainda deve ser no registry ou assim

log msi
https://stackoverflow.com/questions/7126077/create-an-msi-log-file

msiexec /i "Autenticacao.gov_Win_x64_signed (1).msi" /l*v "log.log"

https://github.com/upx/upx/releases/tag/v4.1.0
upx -k c:\Users\mario\Development\Cpp\ConsoleApplicationHelloWorld\x64\Release\ConsoleApplicationHelloWorld.exe

CertMgr
https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/certmgr
