static : hello.o trace_hello.o libhello.a

hello.o : hello.c hello.h
	cc hello.c -c -fPIC -o hello.o

trace_hello.o : 
	dtrace -G -s trace_hello.d hello.o

libhello.a :
	ar crus libhello.a hello.o trace_hello.o 

clean :
	rm hello.o trace_hello.o libhello.a