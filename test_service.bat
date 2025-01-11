sc create event_enricher type=kernel binpath=%CD%\driver\target\debug\event_enricher_package\event_enricher.sys
sc.exe start "event_enricher"
wevtutil.exe im driver\EventEnricher.man
copy "driver\EventEnricherProvider.dll" "C:\Windows\EventEnricherProvider.dll" /Y
timeout /t 2
service\target\debug\service.exe
sc.exe stop "event_enricher"
sc.exe delete "event_enricher"