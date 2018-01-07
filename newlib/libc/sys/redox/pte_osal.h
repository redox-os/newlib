#ifndef _OS_SUPPORT_H_
#define _OS_SUPPORT_H_

#include <sys/types.h>

// XXX
typedef unsigned long pte_osThreadHandle;
typedef unsigned long pte_osSemaphoreHandle;
typedef int32_t* pte_osMutexHandle;

#include "pthreads-emb/pte_generic_osal.h"

#endif // _OS_SUPPORT_H
