#include <sys/sdt.h>
#include "hello.h"

void sayhello(int x)    
{
    DTRACE_PROBE1(myserv, query__receive, x);	
}
