## Code Assistant (Ollama)

A simple system to send commands to an ollama model via http.  Comes in handy
for pipes in helix editor or vim.  This still needs a bunch of work and will
be expanded over time.

### Server:

Simple json based http server to send data to an from ollama model.

```
# Start and run server wherever. 
cd server;
# Set model name in .env file or through env var.
cargo run  
```

### Client:

```
cd clients;
# Install in cargo/bin
cargo install jarvis
```

#### Client usage statement:

```
Usage: stdin | jarvis <COMMAND>

Commands:
  ut    Unit test
  q     inquiry
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help  
```

