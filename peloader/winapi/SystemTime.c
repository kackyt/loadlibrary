#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdbool.h>
#include <search.h>
#include <stdlib.h>
#include <time.h>
#include <assert.h>
#include <sys/time.h>

#include "winnt_types.h"
#include "log.h"
#include "winexports.h"
#include "util.h"

typedef struct _SYSTEMTIME {
  WORD wYear;
  WORD wMonth;
  WORD wDayOfWeek;
  WORD wDay;
  WORD wHour;
  WORD wMinute;
  WORD wSecond;
  WORD wMilliseconds;
} SYSTEMTIME, *PSYSTEMTIME;

typedef struct _TIME_ZONE_INFORMATION {
  LONG       Bias;
  WCHAR      StandardName[32];
  SYSTEMTIME StandardDate;
  LONG       StandardBias;
  WCHAR      DaylightName[32];
  SYSTEMTIME DaylightDate;
  LONG       DaylightBias;
} TIME_ZONE_INFORMATION, *PTIME_ZONE_INFORMATION, *LPTIME_ZONE_INFORMATION;

extern void WINAPI SetLastError(DWORD dwErrCode);

// These routines are called to check if signing certificates have expired, so
// should return similar values.

STATIC VOID WINAPI GetLocalTime(PSYSTEMTIME lpLocalTime)
{
    struct tm local_tim;
    time_t tim;

    time(&tim);
    localtime_r(&tim, &local_tim);

    lpLocalTime->wYear = local_tim.tm_year + 1900;
    lpLocalTime->wMonth = local_tim.tm_mon + 1;
    lpLocalTime->wDayOfWeek = local_tim.tm_wday;
    lpLocalTime->wDay = local_tim.tm_mday;
    lpLocalTime->wHour = local_tim.tm_hour;
    lpLocalTime->wMinute = local_tim.tm_min;
    lpLocalTime->wSecond = local_tim.tm_sec;
    lpLocalTime->wMilliseconds = 0;
}

STATIC VOID WINAPI GetTimeZoneInformation(LPTIME_ZONE_INFORMATION lpTimeZoneInformation)
{
    memset(lpTimeZoneInformation, 0, sizeof(TIME_ZONE_INFORMATION));
    struct timeval tv;
    struct timezone tz;

    gettimeofday(&tv, &tz);

    lpTimeZoneInformation->Bias = tz.tz_minuteswest;

    strncpy(lpTimeZoneInformation->StandardName, "GMT", sizeof("GMT"));

    return;
}

STATIC VOID WINAPI GetSystemTime(PSYSTEMTIME lpSystemTime)
{
    memset(lpSystemTime, 0, sizeof(SYSTEMTIME));
    return;
}

STATIC BOOL WINAPI SystemTimeToFileTime(SYSTEMTIME *lpSystemTime, PFILETIME lpFileTime)
{
    memset(lpFileTime, 0, sizeof(FILETIME));
    return TRUE;
}

STATIC VOID WINAPI GetSystemTimePreciseAsFileTime(PFILETIME lpSystemTimeAsFileTime)
{
    memset(lpSystemTimeAsFileTime, 0, sizeof(FILETIME));
    return;
}

STATIC VOID WINAPI GetSystemTimeAsFileTime(PVOID lpSystemTimeAsFileTime)
{
    memset(lpSystemTimeAsFileTime, 0, sizeof(FILETIME));
    return;
}

STATIC BOOL WINAPI QueryPerformanceCounter(LARGE_INTEGER *lpPerformanceCount)
{
    struct timespec tm;
    DebugLog("");

    SetLastError(0);

    if (clock_gettime(CLOCK_MONOTONIC_RAW, &tm) != 0)
        return FALSE;

    *lpPerformanceCount = tm.tv_nsec;

    return TRUE;
}

STATIC DWORD WINAPI GetTickCount(VOID)
{
    return 0;
}

STATIC ULONGLONG WINAPI GetTickCount64(VOID)
{
    return 0;
}

STATIC BOOL WINAPI QueryPerformanceFrequency(LARGE_INTEGER *lpFrequency)
{
    struct timespec tm;

    DebugLog("");

    if (clock_getres(CLOCK_MONOTONIC_RAW, &tm) != 0)
        return FALSE;

    *lpFrequency = tm.tv_nsec;

    SetLastError(0);

    return TRUE;
}

STATIC BOOL WINAPI GetProcessTimes(HANDLE hProcess, PFILETIME lpCreationTime, PFILETIME lpExitTime, PFILETIME lpKernelTime, PFILETIME lpUserTime)
{
    SetLastError(0);
    DebugLog("");
    return FALSE;
}

STATIC BOOL WINAPI DosDateTimeToFileTime(WORD wFatDate, WORD wFatTime, PFILETIME lpFileTime)
{
    DebugLog("");
    return FALSE;
}

STATIC BOOL WINAPI FileTimeToSystemTime(PFILETIME lpFileTime, PSYSTEMTIME lpSystemTime)
{
    DebugLog("");
    return FALSE;
}

DECLARE_CRT_EXPORT("GetLocalTime", GetLocalTime);
DECLARE_CRT_EXPORT("GetTimeZoneInformation", GetTimeZoneInformation);
DECLARE_CRT_EXPORT("GetSystemTime", GetSystemTime);
DECLARE_CRT_EXPORT("SystemTimeToFileTime", SystemTimeToFileTime);
DECLARE_CRT_EXPORT("GetSystemTimePreciseAsFileTime", GetSystemTimePreciseAsFileTime);
DECLARE_CRT_EXPORT("GetSystemTimeAsFileTime", GetSystemTimeAsFileTime);
DECLARE_CRT_EXPORT("QueryPerformanceCounter", QueryPerformanceCounter);
DECLARE_CRT_EXPORT("QueryPerformanceFrequency", QueryPerformanceFrequency);
DECLARE_CRT_EXPORT("GetTickCount", GetTickCount);
DECLARE_CRT_EXPORT("GetTickCount64", GetTickCount64);
DECLARE_CRT_EXPORT("GetProcessTimes", GetProcessTimes);
DECLARE_CRT_EXPORT("DosDateTimeToFileTime", DosDateTimeToFileTime);
DECLARE_CRT_EXPORT("FileTimeToSystemTime", FileTimeToSystemTime);
