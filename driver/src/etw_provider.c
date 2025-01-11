#include "etw_provider.h"

ULONG _EventRegisterEventEnricher() {
    return EventRegisterEventEnricher();
}

ULONG _EventUnregisterEventEnricher() {
    return EventUnregisterEventEnricher();
}

ULONG _EventWriteProcessNotify(
    GUID* Activity,
    FILETIME* EventTime,
    unsigned __int64 ProcessId,
    PCWSTR FilePath,
    PCWSTR CommandLine
) {
    EventWriteProcessNotify(Activity, EventTime, ProcessId, FilePath, CommandLine);
}

ULONG _EventWriteThreadNotify(
    GUID* Activity,
    FILETIME* EventTime,
    unsigned __int64 ProcessId,
    unsigned __int64 ThreadId
) {
    EventWriteThreadNotify(Activity, EventTime, ProcessId, ThreadId);
}

void _KeQuerySystemTime(PLARGE_INTEGER CurrentTime) {
    KeQuerySystemTime(CurrentTime);
}