[Setup]
; Basic information
AppName=Net Widget
AppVersion=1.0
DefaultDirName={autopf}\NetWidget
DefaultGroupName=NetWidget
OutputBaseFilename=NetWidgetInstaller
Compression=lzma
SolidCompression=yes

[Files]
; Copy the executable to the Program Files directory
Source: "target\release\NetWidget.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
; Create a shortcut in the Startup folder pointing to the executable in Program Files
Name: "{userstartup}\Net Widget"; Filename: "{app}\NetWidget.exe"

[Run]
; Optionally, run the application after installation
Filename: "{app}\NetWidget.exe"; Description: "Launch Network Speed Widget"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Delete the executable from Program Files during uninstallation
Type: files; Name: "{app}\NetWidget.exe"
; Delete the shortcut from the Startup folder during uninstallation
Type: files; Name: "{userstartup}\Net Widget.lnk"
