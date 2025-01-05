sc create EventEnricher type=kernel binpath=%CD%\target\debug\EventEnricher_package\EventEnricher.sys
sc start EventEnricher
sc stop EventEnricher
sc delete EventEnricher