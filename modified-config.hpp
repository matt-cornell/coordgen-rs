#pragma once

#ifndef STATIC_COORDGEN

#ifdef _MSC_VER
#ifdef IN_COORDGEN
#define EXPORT_COORDGEN __declspec(dllexport)
#else
#define EXPORT_COORDGEN __declspec(dllimport)
#endif // IN_COORDGEN

#else

#define EXPORT_COORDGEN __attribute__((visibility("default")))

#endif // _MSC_VER

#else

#define EXPORT_COORDGEN

#endif // STATIC_COORDGEN
