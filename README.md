# event_enricher
Driver and service which help enrich event logs for detection purposes

# Build
All testing was done on Windows 11 with the following dependencies installed:
```
winget install --id Git.Git -e --source winget
winget install --id LLVM.LLVM -e --source winget
winget install --id Rustlang.Rustup -e --source winget
rustup install nightly
rustup default nightly
winget install --id Microsoft.VisualStudio.2022.Community --source winget --override "--add Microsoft.VisualStudio.Workload.NativeDesktop --add Microsoft.VisualStudio.Workload.Universal --includeRecommended --includeOptional"
winget install --source winget --exact --id Microsoft.WindowsWDK.10.0.22621
cargo install --locked cargo-make --no-default-features --features tls-native
```
Assuming they have been installed, you can build the driver by:
- Starting a "Developer Command Prompt for VS 2022" terminal or execute the following within a command prompt:
    ```
    C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat
    ```
- Execute `cargo make`

This will result in the driver package being within the `driver\target\debug\event_enricher_package` directory.

# Execution
The driver is not signed and so test signing must be enabled. To enable test signing, open a command prompt as Administrator and execute:
```
bcdedit /set testsigning on
```
Restart your computer to apply the changes.

To install the driver, open a command prompt as Administrator and execute:
```
sc create event_enricher type=kernel binPath=<path_to_driver_sys>
sc start event_enricher
```
Replace `<path_to_driver_sys>` with the path to the driver's `.sys` file.

Once installed, go ahead and place `driver\EventEnricherProvider.dll` into `C:\Windows\EventEnricherProvider.dll` followed by executing the following command in order to install the manifest for the custom event logs:

```
wevtutil.exe im driver\EventEnricher.man
```

Since the driver should now be generating logs, go ahead and execute `service\target\debug\service.exe` which will output the events being created by the driver as seen below:
```
Process Start:
        PID: 2428,
        Image: \??\C:\Users\user\.cargo\bin\rustfmt.exe,
        Cmd: "rustfmt"
Thread Start:
        PID: 2428,
        TID: 5836
Thread Start:
        PID: 2428,
        TID: 1632
Thread Start:
        PID: 2428,
        TID: 1680
Process Start:
        PID: 5424,
        Image: \??\C:\Users\user\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustfmt.exe,
        Cmd: "C:\Users\user\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustfmt.exe"
```

# Removal
To delete the driver, open a command prompt as Administrator and execute:
```
sc stop event_enricher
sc delete event_enricher
```

To disable test signing, open a command prompt as Administrator and execute:
```
bcdedit /set testsigning off
```

To remove the manifest delete `C:\Windows\EventEnricher.dll` and execute the following command:

```
wevtutil.exe um driver\EventEnricher.man
```

Restart your computer to apply the changes.

# Inspiration/References
- https://github.com/jsecurity101/JonMon
- https://github.com/0xflux/Sanctum
- https://codentium.com/guides/windows-dev/