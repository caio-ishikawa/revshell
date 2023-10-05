RevShell v1.0.0
---------------
Simple tool to generate reverse shells in the terminal. Supports command line flags for IP address, port number, and shell type.

Installation
---------------
### Using cargo
If Cargo is installed in your machine, RevShell can be installed with the following command:
```sh
cargo install revshell
```

### Building From Source
This tool is written in [Rust](https://www.rust-lang.org/), and you will need to install the Rust language/compiler/toolkit if you don't already have it. Full details of installation and set up can be found [on the Rust language website](https://www.rust-lang.org/tools/install). Once installed you can run the following command:

```sh
git clone https://github.com/caio-ishikawa/revshell.git && cd revshell && make install 
```

### Uninstalling
To uninstall RevShell, you can navigate to the cloned repository (or clone it if yo have deleted it), and run:

```sh
make uninstall
```

Getting Started
---------------
```
Usage: revshell [OPTIONS]

Options:
  -s, --shell <SHELL>     Represents the shell (e.g. bash_-i, python3, etc.)
  -o, --output <OUTPUT>   Represents the name of the output file. If this flag is not set, the script will be printed to stdout
  -p, --port <PORT_NUM>   Port number
  -i, --ip <IP_ADDRESS>   IP address
      --supported-shells  Displays all supported shells
  -h, --help              Print help
  -V, --version           Print version
```

### Example:
```sh
revshell -s bash_-i -i 00.00.00.00 -p 9000 -o shell.sh # this will create a file called shell.sh in the current directory.
```

