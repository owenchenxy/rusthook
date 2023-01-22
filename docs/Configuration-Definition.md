# Configuration Definition
Hooks are defined in a configuration file of yaml format. All properties have a default value, but in order to let the hook make sense, users should at least define `id` and `execute_command` by themselves.

## Properties(Keys)
### Global Configuration

### Hook Configuration
+ `id` - specifies the ID of the hook. Rusthook server will create an endpoint using this ID. for example, with the id of `myid`, the server will be able to receive request on url `http:://your_server:port/myid`
+ `execute_command` - specifies the command to be executed when the hook of corresponding id is triggered
+ `command_working_directory` - specifies the directory, to which should be switched while executing command
+ `response-message` - specifies the value of the `message` propertie in the json to be returned to the hook initiator
+ `response-headers`(to be supported) - specifies the list of headers in format {"name": "X-Example-Header", "value": "it works"} that will be returned in HTTP response for the hook
+ `pass-arguments-to-command` - specifies a list of arguments for the command. Check [Referencing Request Values As Parameter page](Referencing-Request-Values-As-Parameter.md) to see how to reference the values as command parameter from the request.