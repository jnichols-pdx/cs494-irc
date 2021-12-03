# CS494 IRC Chat System

Yet Another Rust IRC Client - A client and server implementation of a custom internet relay chat protocol.
This client and server are _not_ compatible with RFC 1459. The chat client uses the Cursive library to
offer a basic terminal user interface (TUI).

## Author and Assignment

This is the result of an undergraduate programming assignment

* by: James Nichols, jtn4@pdx.edu
* for: CS494p Internetworking Protocols, Fall 2021 Term Project
* adapted from: an example chat protocol RFC provided in course materials

## IRC (sort of)

Internet Relay Chat allows users to connect chat clients to a central chat server.
The server hosts any number of chat rooms, where users may share text messages visible
by other users in the same chat room. This project implements a custom network protocol
for information sent between the chat client and server. This is _not_ the internet
standard IRC protocol as defined in RFC 1459. See the file CS494_IRC_RFC_JTN4.pdf for
protocol details.

## Compiling

The client and server may be compiled with the command `cargo build`.

A suite of unit tests may be run against the library code shared in common by the server
and client with the command `cargo test`. 

## Usage - server

The chat server automatically listens on port 17734 for connections from chat clients.

The server may be started via cargo with the command `cargo run --bin server`
The server may be stopped by pressing ctrl-c on the commandline.

## Usage - client

The client may be started via cargo with the command

`cargo run --bin client <username> [server]`

By default the client attempts to connect to localhost on port 17734.

`<username>` is mandatory and sets your IRC username / handle which is visible to other chat users.

Usernames may not contain spaces and are limited to the lesser of 32 unicode characters or 64 utf-8 encoded bytes.

The server will reject a client connection if the chosen username is already in use.

`server` is optional and takes the form `ip-address:port`
This specifies the IP address and port to which the client should connect to when looking for an IRC server.
While the client may specify a custom port, the server is currently hard coded to only listen on port 17734.

The client may be closed by pressing either ctrl-c or ctrl-q.

### Navigation

The Cursive UI used by the client allows for navigating by either mouse or arrow keys.

You may click on buttons, names of rooms in the room list, tabs, or the input lines directly.

You may move the highlight to buttons, names of rooms in the room list, or tabs and press the Enter key

to make selections. The enter key also enters commands or sends messages to the current chat tab.

Tabs may also be switched by pressing ctrl + Right Arrow or ctrl + Left Arrow

### Text Commands

The text input line of chat rooms and direct messaging tabs understands certain words with a slash in front
of them as commands:

* /join <roomname> - Joins your user to a chat room, creating a new chat tab for that room.
* /enter <roomname> - Same as /join
* /whisper <username> - Starts a Direct Message session with another user
* /tell <username> - Same as /whisper
* /status <username> - Asks the chat server if a given user is online
* /leave - Removes your user from the current chat room or direct message sesion, closing the tab.
* /yell <message text> - Posts a message to All chat rooms that your user is in.
* /broadcast <message text> - same as /yell
* /offer <username> <filename> - NOT IMPLEMENTED - Intended to initiate a file transfer to another user.

# Known Bugs:

There is a bug in either the library crate which provides Tabbed views in Cursive, or how this client 
interacts with them. If one switches to a chat tab that is _not_ at the right end of the list with the 
ctrl + arrow key shortcuts, /leaves that chat window, and then clicks on one of the remaining tabs the
UI thread may crash. To avoid this error one should first switch to a tab they wish to close either by
clicking on its name, or by highlighting its tab with the arrow keys and pressing Enter. If you are
currently on the tab for a conversation you wish to /leave this may mean require switching to it again
just before leaving the channel.
