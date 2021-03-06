# MKProject Package Manager

The MKProject Package Manager or MKPM in short is a Package Manager built for
the various projects in MKProject. This project works by having a remote server (`mkpm_server`)
and a client-side CLI application (`mkproj_client`). These two projects share 
a library (`mkproj-sl`) that gives the needed structs and methods for both client 
and server-side applications. 

## Installation
You can instal via Cargo: 
```shell
$ cargo install mkpm-cl # This is the client-side application
```

## Commands
In `mkpm-cl` there are four main commands: 
1. `get`: This fetches an installer for applications such as `mkproj_book`
2. `install`: This installs any Cargo project
3. `clone`: Clones a MKProject repo 
4. `read`: Grabs a pdf of a MKProject Book

For a list `list --cmd <command>` will display the available items for a specified command. 

## Running Locally 
```shell
# First clone the project 
$ git clone http://github.com/MKProj/Package_Manager.git
# Change directory into Package Manager
$ cd Package_Manager
# In a seperate window, run the server 
$ cd mkpm_server && cargo run --release 
# In a seperate window, first edit the `client.toml` file
``` 
```toml
# mkpm_client/client.toml
addr = "127.0.0.1:8000/"
```
```shell
# Now run the client 
$ cd mkpm_client && cargo run --release -- <command>
```