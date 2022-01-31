# MKProject Package Manager

The MKProject Package Manager or MKPM in short is a Package Manager built for
the various projects in MKProject. This project works by having a remote server (`mkpm_server`)
and a client-side CLI application (`mkproj_client`). These two projects share 
a library (`mkproj-sl`) that gives the needed structs and methods for both client 
and server-side applications. 

## Installation
You can instal via Cargo: 
```shell
$ cargo install mkpm # This is the client-side application
```