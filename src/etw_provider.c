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
    PCWSTR FilePath,
    PCWSTR CommandLine,
    unsigned __int64 ProcessId
) {
    EventWriteProcessNotify(Activity, EventTime, FilePath, CommandLine, ProcessId);
}

ULONG _EventWriteThreadNotify(
    GUID* Activity,
    FILETIME* EventTime,
    unsigned __int64 ProcessId,
    unsigned __int64 ThreadId
) {
    EventWriteThreadNotify(Activity, EventTime, ProcessId, ThreadId);
}