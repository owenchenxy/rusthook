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

+ `hmac-sha1`: ToBeDone
+ `hmac-sha256`: ToBeDone
+ `hmac-sha512`: ToBeDone
+ `ip-whitelist`: ToBeDone
    
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