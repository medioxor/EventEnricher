sc create event_enricher type=kernel binpath=%CD%\driver\target\debug\event_enricher_package\event_enricher.sys
sc start event_enricher
timeout /t 2 /nobreak
service\target\debug\service.exe
sc stop event_enricher
sc delete event_enricher