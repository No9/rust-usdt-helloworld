#include <sys/sdt.h>
#include "libhello.h"  /* Include the header (not strictly necessary here) */

int sayhello(int x)    /* Function definition */
{
    DTRACE_PROBE2(myserv, query__receive, "data", x);	
    return x + 5;
}
