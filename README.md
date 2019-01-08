# rust-usdt-helloworld

A basic example of Userland Statically Defined Tracing in Rust using native C.

Inspired by [rust-libprobe](https://github.com/cuviper/rust-libprobe) but that is Linux focused and uses `asm!` which is currently only available in nightly.

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

## usage 

This should "Just Work"&trade;

```
% cargo run
```

As root run the dtrace script to see the probes
```
# dtrace -l | grep myserv
73093 myserv3361 rust-usdt-helloworld                          sayhello query-receive
```

Now run the a dtrace to consume the probe.
```
# dtrace -s tools/probe.d
sayhello Fired 3
sayhello Fired 4
sayhello Fired 5
...
```


## notes

The trick here is in the `build.rs` where the trace provider object is generated using the .d script and the "mungled" hello object

```
% dtrace -G -s trace_hello.d libhello.o
```
