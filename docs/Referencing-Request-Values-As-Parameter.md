# Referencing Request Values As Parameters
1. Refer from Http Headers, e.g. 
```
{
  "source": "header",
  "name": "Access-Control-Allow-Origin"
}
```
2. Refer from Http Query Parameters, e.g. for a GET query looks like: "http://yourserver:port/hook/?param1=val1&param2=val2", to refer param1:  
```
{
  "source": "url",
  "name": "param1"
}
```
3. Refer from Http Request Parameters. Only support two types:
```
{
  "source": "request",
  "name": "Method"
}
```
```
{
  "source": "request",
  "name": "Peer-Address"
}
```
4. Refer from Http Payload(Http body of a POST request)
```
{
  "source": "payload",
  "name": "parameter-name"
}
```
payload is assumed to be json encoded. For example, if a POST request body looks like follow:
```
{
    "user_infos": [
        {
            "id": 1,
            "name": "alex"
        },
        {
            "id": 2,
            "name": "bob"
        }
    ]
}
```
string `alex` can be referred by:
```
  "source": "payload",
  "name": "user_infos.0.name"
```
if the payload looks like this:
```
{
    "user_infos.0.name": "prior" ,  
    "user_infos": [
        {
            "id": 1,
            "name": "alex"
        },
        {
            "id": 2,
            "name": "bob"
        }
    ]
}
```
Then the value of key `user_infos.0.name` will refer string `prior` instead of `alex`. The server will preferentially consider a key as direct one rather than a nested one combined by the dot-notation.
5. Refer the entire payload:
```
  "source": "payload",
  "name": "entire-payload"
```
Set as above to refer the entire payload as command parameter. 