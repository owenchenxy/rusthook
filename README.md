# What is rusthook ?

<img src="https://github.com/owenchenxy/rusthook/blob/main/docs/logo.png" alt="Rusthook" align="left" />

rusthook is a lightweight configurable webhook tool written in rust. It allows you to run a http server with specified endpoints(hooks), so that you can execute configured commands by sending http request to them. You can also configure the command to be exeuted with arguments, which can be parsed from the http request(e.g. headers, payload, query variables), or directly specified by a string.

For example, if you're using a monitoring tool like Grafana, you can configure an alert rule to use webhook notifier, which will trigger a HTTP request(GET/POST) to a specified url, so that required actions would be performed upon the alert.

webhook should only focus on limited functions which is listed below:

+ receive the request,
+ parse the request(headers, body, query variables),
+ check if the requested hook id matches any of the configuration item,
+ parse arguments from the request according to the configuration,
+ check the trigger rules to determine whether to trigger hook,
+ execute the specified command with arguments if rules match,
+ send response to the request initiator
  
Everything else is the responsibility of the command's author.

# Getting Started
## Installation
### Building from source
The project is initiated with `cargo version 1.65.0`, which is suggested to set. Clone this git repo to your local workstation and then run
```
cargo build --release
```
to build the latest version of rusthook.

# Configuration
Next step is to define a list of hooks you want rusthook to serve. Currently rusthook only support configuration file of YAML format.Begin by creating an empty file named hooks.yaml. This file will contain a list of endpoints(hooks) to be served.

For purpose of managing several global configurations like logs location/prefix, a global config section is neccessary, which looks like below:
```
---
global:
  log_dir: "logs" 
  log_prefix: "webhook"
  log_level: "Info"
```
In current release, all global configs are log related and they all have default values, so that they can be ommited. To use default global configs, the global config looks simple like this:
```
---
global:
```

Now, let's configure the real useful hooks. Imagine there is a cpu monitor in my system, it will send a http request to a webhook upon high CPU utilization detected. When the hook server receives the request, it knows that there's something wrong with the CPU utilization and should perform some actions to repair it. 

For this senario, let's define a simple hook named `cpu_high_alert`(endpoint to receive alert request) that will run a script located in `/var/scripts/repair_cpu_high.sh`(to repair something according to the alert). Make sure that your bash script has `#!/bin/sh` shebang on top and is executable.

Our hooks.yaml file will now look like this:
```
---
global:
  log_dir: "logs" 
  log_prefix: "webhook"
  log_level: "Info"
hooks:
  - id: cpu_high_alert
    execute-command: "/var/scripts/repair_cpu_high.sh"
    command-working-directory: "/var/webhook"
```

rusthook supports configuring multiple endpoints. For example, in the senario above, we also want to do something when high memory utilization is detected. We can add a configuration item, which makes the entire configuration files look like this:
```
---
global:
  log_dir: "logs" 
  log_prefix: "webhook"
  log_level: "Info"
hooks:
  - id: cpu_high_alert
    execute-command: "/var/scripts/repair_cpu_high.sh"
    command-working-directory: "/var/webhook"
  - id: mem_high_alert
    execute-command: "/var/scripts/repair_mem_high.sh"
    command-working-directory: "/var/webhook"
```

You can now run rusthook using
```
/path/to/rusthook --config hooks.yaml
```
It will start up on any ip of your server with default port 7878 and will provide you with HTTP endpoints configured:
```
1. http://yourserver:7878/cpu_high_alert
2. http://yourserver:7878/mem_high_alert
```

Check [rusthook parameters page](docs/Rusthook-Parameters.md) to see how to override the ip, port and other performance settings such as maximum thread number, tread stack size, etc.

By performing a simple HTTP GET or POST request to those endpoint, your specified repair script would be executed!

Furthermore, you can define a combination of rules to determine whether the hook would be triggered. This is absolutely for demand of security. Without the rules, anyone who knows your endpoint can send a request to your server and thus execute the command. To define the rules, you can use the `trigger_rules` property for a specific hook. Please refer to doc [Hook Trigger Rules page](docs/Hook-Trigger-Rules.md) to see the detailed list of available rules and their usage.
