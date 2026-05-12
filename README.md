# Splatted 🦑
Splatted is a lightweight command-line utility written in Rust for interacting with Minecraft servers. It focuses on low-level packet handling and asynchronous communication to stress test the server by sending lots of connections
## How to use it 
First of all, make sure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

## Installation (Optional, you can download the precompiled binaries from the releases section)
```bash
git clone https://github.com/hthug06/splatted.git
cd splatted
cargo build --release
```

## Usage
Go in the folder where your Splatted program is, open a terminal and use the following command:

```bash
./Splatted --address <address> [-p <port> | -i | -b <bot_number> | -r <protocol>]
```
-i is for the info of the server (like in the server list of your Minecraft client)


| Arguments          |  Short   | Default           |                                                                       Description                                                                       |
|--------------------|:--------:|-------------------|:-------------------------------------------------------------------------------------------------------------------------------------------------------:|
| ```--address```    |    -     | **Required**      |                                                                The server IP or hostname                                                                |
| ```--port```       | ```-p``` | ```25565```       |                                                                 The target server port                                                                  |
| ```--info```       | ```-i``` | ``` false```      |                                                    See like in the server list of a minecraft client                                                    |
| ```--bot_number``` | ```-b``` | ``` 1```          |                                                    The number of bots you want to send to the server                                                    |
| ```--protocol```   | ```-r``` | ``` 51 (1.4.7)``` | The protocol version of the minecraft server (click [here](https://minecraft.wiki/w/Protocol_version#Java_Edition_(pre-netty_rewrite)) and scroll down) |

## Roadmap
- [x] Get the server infos like in the multiplayer section of a real client
- [x] Good structure to read / send a packet
- [x] Connect one single bot (pls)
- [x] Send bots to a server and make them disconnect
- [x] Make version 1.2.5 to 1.6.4 work

## Other
Thx [tetram](https://github.com/tetram2674562) ur the goat :)