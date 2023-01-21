# Rusthook Parameters
```
Usage: rusthook [OPTIONS]

Options:
  -i, --ip <IP>                  the ip on which the server is listening [default: 0.0.0.0]
  -p, --port <PORT>              the port on which the server is listening [default: 7878]
  -c, --config <CONFIG>          config file path [default: src/config/hooks.test.yaml]
  -t, --threads <THREADS>        max number of threads [default: 4]
  -s, --stack-size <STACK_SIZE>  stack size for each thread [default: 4000000]
  -h, --help                     Print help
  -V, --version                  Print version
```
Use the above specified flags to override the default values.