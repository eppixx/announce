This crate uses announce-lib to build a CLI application for messaging and sending files to supported services.

# Supported services
Currently supported services are:
* Rocket.Chat
* D-Bus
* Discord

# How to use
A help page can be displayed by using announce with `--help`.

## Choosing a target service
The services and the information to use them are stored in URI format. The scheme indicates which service is the target.
The rest of the format depends on the service chosen.

## Messages
It is possible to send Messages in text, links and files.
* `-m` is used for messages
* `-f` is used for files
Please keep in mind, that there are flags that aren't supported by all services.
When using such a combination an error will be displayed.
