![Crab](crab.png)
# drunkfinder
Simple CRC32 brute forcer written in Rust that by default tests for strings that evaluate to 5162020, a secret world seed known as "Drunk World" in the sandbox computer game Terraria. 

Provide a wordlist named "wordlist" (without extension) in the same directory as Cargo.toml.

## Requirements
See Cargo.toml.

### Hardware
This crate uses the crc32fast crate. The crate crc32fast uses SIMD instructions (SSE) found on x86 processors. 

So far I have only tested this on a x86-64 FreeBSD 13.1 VM hosted on a dual Intel Xeon E5640 system. For now, only one thread is used so single-thread performance determines the calculation speed. Update, December 9, 2022: Tested Windows 10 on an AMD Ryzen 5 5600.

A python script, split.py, is included to split large wordlists into smaller even lists that can be used while running multiple instances.
