# drunkfinder
Simple CRC32 brute forcer written in Rust that by default tests for strings that evaluate to 5162020, a secret world seed known as "Drunk World" in the sandbox computer game Terraria. 

## Requirements
See cargo.lock

This crate uses the crc32fast crate. The crate crc32fast uses SIMD instructions (SSE) found on x86 processors. 

So far I have only tested this on a x86-64 FreeBSD 13.1 VM hosted on a dual Intel Xeon E5640 system. So far, only one thread is used.
