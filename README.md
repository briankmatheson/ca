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
bmath@gram:~/ca$ openssl x509 -in foo.bmath.nyc.pem -text
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            76:ae:94:d3:1f:91:23:3b:b4:28:6e:44:5a:87:aa:16:3e:e3:58:d4
        Signature Algorithm: ecdsa-with-SHA256
        Issuer: CN=CA
        Validity
            Not Before: Jan  1 00:00:00 1975 GMT
            Not After : Jan  1 00:00:00 4096 GMT
        Subject: CN=foo.bmath.nyc
        Subject Public Key Info:
            Public Key Algorithm: id-ecPublicKey
                Public-Key: (256 bit)
                pub:
                    04:5b:f0:62:af:07:61:61:23:16:f5:17:b9:97:02:
                    f0:c1:00:24:9c:fe:29:b6:41:a6:26:0d:42:58:b2:
                    e4:8f:8f:65:48:d4:38:7d:bd:b3:78:ef:f1:b3:d3:
                    37:79:5b:d1:85:60:4a:cd:38:34:6d:65:36:15:33:
                    09:92:f0:3d:11
                ASN1 OID: prime256v1
                NIST CURVE: P-256
        X509v3 extensions:
            X509v3 Subject Alternative Name: 
                DNS:foo.bmath.nyc
    Signature Algorithm: ecdsa-with-SHA256
    Signature Value:
        30:46:02:21:00:fb:98:3d:26:ff:e1:41:71:ed:c8:04:db:47:
        88:aa:4f:9c:06:b1:fe:fb:05:5c:5a:a2:bc:18:45:bd:0a:bb:
        a9:02:21:00:a8:32:44:0c:70:dc:37:42:d7:d9:72:48:e0:bb:
        a2:0b:3f:fe:2d:19:80:d9:eb:3c:d3:0c:8c:df:ab:4b:e6:77
-----BEGIN CERTIFICATE-----
MIIBRTCB66ADAgECAhR2rpTTH5EjO7QobkRah6oWPuNY1DAKBggqhkjOPQQDAjAN
MQswCQYDVQQDEwJDQTAgFw03NTAxMDEwMDAwMDBaGA80MDk2MDEwMTAwMDAwMFow
GDEWMBQGA1UEAxMNZm9vLmJtYXRoLm55YzBZMBMGByqGSM49AgEGCCqGSM49AwEH
A0IABFvwYq8HYWEjFvUXuZcC8MEAJJz+KbZBpiYNQliy5I+PZUjUOH29s3jv8bPT
N3lb0YVgSs04NG1lNhUzCZLwPRGjHDAaMBgGA1UdEQQRMA+CDWZvby5ibWF0aC5u
eWMwCgYIKoZIzj0EAwIDSQAwRgIhAPuYPSb/4UFx7cgE20eIqk+cBrH++wVcWqK8
GEW9CrupAiEAqDJEDHDcN0LX2XJI4LuiCz/+LRmA2es80wyM36tL5nc=
-----END CERTIFICATE-----

```
