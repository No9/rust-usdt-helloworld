# rust-usdt-helloworld

A basic example of Userland Statically Defined Tracing in rust

Other projects that use DTrace in rust depend on `asm!` which is currently only available in nightly.

This is a very basic sample of how to create a native c dtrace probe and include it into a rust project built with stable.

It's based on the sample given in the [dtrace.org website](http://dtrace.org/guide/chp-usdt.html) 

## prereqs

```
% uname -a 
FreeBSD  11.1-RELEASE FreeBSD 11.1-RELEASE #0: Sun Dec 31 13:23:21 UTC 2017     root@:/usr/obj/usr/src/sys/FIFOKERNEL  amd64
% pkg install rust
% rustc -V
rustc 1.23.0
% cc --version
FreeBSD clang version 4.0.0 (tags/RELEASE_400/final 297347) (based on LLVM 4.0.0)
Target: x86_64-unknown-freebsd11.1
Thread model: posix
InstalledDir: /usr/bin
```

## build 

Build the native object

```
% cd native 
% cc -c libhello.c
```

Generate the trace provider object using the .d script and the libhello object

```
% dtrace -G -s trace_hello.d libhello.o
```

Build the shared object

```
% cc trace_hello.o libhello.o -shared -o libhello.o.so 
```

Build the rust app

```
% cd ../src
% rustc hello.rs -L ../native/
```

## run

```
% cp ../native/libhello.o.so libhello.so
% set env LD_LIBRARY_PATH .
% ./hello
```

## see the probe

In a new shell as root list the running myserv probes

```
# dtrace -l | grep myserv
69643 myserv37329       libhello.so                          sayhello query-receive
```
