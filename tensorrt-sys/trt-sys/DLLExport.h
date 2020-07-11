#ifndef __DDLEXPORT_H
#define __DDLEXPORT_H

#if defined(_WIN32) || defined(WIN32) || defined(_WIN64) || defined(WIN64) 
    #define DllExport   __declspec( dllexport )
#else
    #define DllExport
#endif

#endif