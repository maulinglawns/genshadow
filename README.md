# genshadow
Simple command line utility for generating sha512 hash

```
genshadow 0.1.2
Magnus Wallin <magnuswallin@tutanota.com>
A simple compiled command line utility for generating a sha512 hash (for use in /etc/shadow).
Exit codes:
0: Success
Non-zero: Failure

USAGE:
    genshadow

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

Example:
```
./genshadow 
Enter your password: 
Please verify your password: 
Your sha512 hash:
$6$alnsK3Kmj2FWJ/h2$uc/v7Fgmv.kd6aWimDG7b7vL/UnOe8TnKETI4wdqJAb/uRL7jXaKIgwtVcOEwzEgGeJrYKHOouSiAyCsPNzdl/
```
