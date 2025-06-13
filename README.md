ca
==
Simple tool to generate and sign certs

```
bmath@gram:~$ mkdir -m 700 ca
bmath@gram:~$ cd ca
bmath@gram:~/ca$ ca new
bmath@gram:~/ca$ ca server foo.bmath.nyc
bmath@gram:~/ca$ ca client bmath.bmath.nyc
bmath@gram:~/ca$ ll
total 24K
-rw-rw-r-- 1 bmath bmath 241 Jun 13 04:16 ca.key
-rw-rw-r-- 1 bmath bmath 562 Jun 13 04:16 ca.pem
-rw-rw-r-- 1 bmath bmath 501 Jun 13 04:16 foo.bmath.nyc.pem
-rw-rw-r-- 1 bmath bmath 241 Jun 13 04:16 foo.bmath.nyc.key
-rw-rw-r-- 1 bmath bmath 534 Jun 13 04:16 bmath.bmath.nyc.pem
-rw-rw-r-- 1 bmath bmath 241 Jun 13 04:16 bmath.bmath.nyc.key
```
