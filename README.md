# Morse-Chat-Server

Morse Char Server is a web-server written in Rust using Rocket.rs web framework.

This server features two services:
- UsersService -> A service that allows users registering
- MessageService -> A service that allows users to create & retrieve messages

Whan an user sends a new message, the message will be broadcasted to all other users.

The intended usage of this service is to allow a MicrobitV2 client to broadcast messages
to all other clients. Due to limitations on the client's user interface - Microbitv2 only
features two buttons and a touch button to interact with the UI - this service was configured
only to allow broadcasts, but may be improved to allow private conversations.

Finally, this project is for learning purposes and showcases an example of how to create
a Rust webapp using Rocket.rs

