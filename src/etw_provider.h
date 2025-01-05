#define WIN32
#define _NTDRIVER_
#define _AMD64_
#define _WINNT_
#define _ETW_KM_
//#include <ntddk.h>
#include <wdm.h>

#define MCGEN_USE_KERNEL_MODE_APIS 1

typedef unsigned short WORD;
typedef unsigned long DWORD;

typedef struct _SYSTEMTIME {
	WORD wYear;
	WORD wMonth;
	WORD wDayOfWeek;
	WORD wDay;
	WORD wHour;
	WORD wMinute;
	WORD wSecond;
	WORD wMilliseconds;
} SYSTEMTIME, * PSYSTEMTIME, * LPSYSTEMTIME;

typedef struct _FILETIME {
	DWORD dwLowDateTime;
	DWORD dwHighDateTime;
} FILETIME, * PFILETIME, * LPFILETIME;

#include "../EventEnricher.h"