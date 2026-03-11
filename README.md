# Grave

This is my attempt at a shellcode loader in Rust (a language I have little to no experience developing in). Definitely was a learning experience but thankfully most of the stuff I learned previously about maldev carries over rather easily.

## Features
 - Compile time IAT entries
 - Compile time string obfuscation
 - Dynamic WinAPI function resolution with compile time function name hashing
 - Mutex
 - Fetching shellcode from a remote host
 - HTTP/S support
 - Earlybird shellcode injection
 - Anti-analysis/vm/virustotal via Username

## Building
```Powershell
cargo +nightly build --release -Z build-std=std,panic_abort --target x86_64-pc-windows-msvc
```

## Credit:
 - Maldev Academy (Early Bird injection, IAT obfuscation/stuffing)
 - Opencode (Debugging and figuring out what I had written at 4 am previously)
 - https://github.com/6nz/virustotal-vm-blacklist/blob/main/pc_username_list.txt (Usernames List)
