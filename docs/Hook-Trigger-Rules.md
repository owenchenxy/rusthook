# Hook Trigger Rules
## Single Rule
Every single rule are defined by 4 keys:
1. `kind`: the type of the rule
2. `value`: the expression to be compared with the data from `source`
3. `source`: where the data comes from
4. `name`: the name of data, as the index to find from `source`

### Supported kind
+ `value`: source data's value exactly equals to the rule's `value` field.
    ```  
    kind: value
    value: 127.0.0.1:7878
    source: header
    name: Host
    ```
    the above rule will be evaluated to be true, if the value of header `Host` in http request exactly equals to `127.0.0.1:7878`

+ `regex`: source data's value matches the regular expression defined in the rule's `value` field.
    ``` 
    kind: regex
    value: ".*:7878"
    source: header
    name: Host
    ```
    the above rule will be evaluated to be true, if the value of header `Host` in http request matches the regular expression `.*:7878`

+ `hmac-sha1`: payload will be encrypted by the give secret using SHA1 hash and compare with the specified source signature. The field `value` is for the secret key for encryption.
    ```
    kind: hmac-sha1
    value: "mysecret"
    source: header
    name: "X-Signature"
    ```
    for the above rule, with payload of below:
    ```
    {
      "payload": "mypayload"
    }
    ``` 
    we can calculate it's sha1 hash with secret "mysecret", it should be `22b5b8548adfdec0322b2114f17648c34a3081e6`. Thus in the request headers, header "X-Signature" should contain a signature with this hash. e.g. 
    ```
    X-Signature: sha1=22b5b8548adfdec0322b2114f17648c34a3081e6
    ```
    Note that if there are multiple signatures, they should be seperated by comma and will be tried one by one until a match is found. For example:
    ```
    X-Signature: sha1=232328349ffdaccc78999897,sha1=22b5b8548adfdec0322b2114f17648c34a3081e6
    ```
+ `hmac-sha256`: similar to `hmac-sha256`, payload will be encrypted by the give secret using SHA256 hash and compare with the specified source signature. The field `value` is for the secret key for encryption.
    ```
    kind: hmac-sha256
    value: "mysecret"
    source: header
    name: "X-Signature"
    ```
+ `hmac-sha512`: similar to `hmac-sha512`, payload will be encrypted by the give secret using SHA512 hash and compare with the specified source signature. The field `value` is for the secret key for encryption.
    ```
    kind: hmac-sha512
    value: "mysecret"
    source: header
    name: "X-Signature"
    ```
+ `ip-whitelist`: only allow requests sent from the addresses matching the ip whitelist. The IP can be IPv4/6 formatted. Use /32(/128) to match a single IPv4(6) address. field `source` and `name` can be ommited. Multiple ip ranges could be seperated by comma.
    ```
    kind: ip-whitelist
    value: "10.0.0.0/24, 192.168.100.4/30, 172.16.80.3/32"
    ```
    
## Combined Rule
Combined rule is combined by multiple single rule or combined rule with logical operations(And/Or/Not).

### And
And rule will evaluate to true only if all of the sub rules evaluate to true.
```
and:
  - kind: value
    value: 127.0.0.1:7878
    source: header
    name: Host
  - kind: regex
    value: "superuser: 0[0-9]*"
    source: payload
    name: user.id
```
## Or
Or rule will evaluate to true, if any of the sub rules evaluate to true.
```
or:
  - kind: value
    value: 127.0.0.1:7878
    source: header
    name: Host
  - kind: regex
    value: "superuser: 0[0-9]*"
    source: payload
    name: user.id
```
## Not
Not rule will evaluate to true, if it's sub rule evaluate to false.
```
not:
  kind: value
  value: "yes"
  source: payload
  name: control.disabled
```
## Multi-Level 
Rules can be defined in multiple level
```
and: 
  - kind: value
    value: 127.0.0.1:7878
    source: header
    name: Host
  - not:
      kind: value
      value: "yes"
      source: payload
      name: control.disabled
  - or:
      - kind: regex
        value: ".* Smith"
        source: payload
        name: user.name
      - kind: regex
        value: "superuser: 0[0-9]*"
        source: payload
        name: user.id
```
