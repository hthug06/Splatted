[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=plastic&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/github/license/hthug06/Splatted?style=social)](https://github.com/hthug06/Splatted/blob/main/LICENSE)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/hthug06/Splatted?style=social)]()

<div align="center">

# Splatted 🦑

Splatted is a lightweight command-line utility written in Rust for interacting with Minecraft servers.
It focuses on low-level packet handling and asynchronous communication to stress test the server by sending lots of connections.

</div>

---

## Why Splatted?
- **Rust-Powered**: Built with Rust for performance, safety, and reliability.
- **Multi-version Support**: Fully supports and connects bots to Minecraft versions from **1.2 to 1.6.4**.
- **Server Ping (Info)**: Retrieve server details directly from the command line, just like the multiplayer server list in a real client.
- **Stress Testing**: Efficiently send single or multiple bots to a target server and manage their disconnections.
- **Low-level Packets**: Custom, streamlined structure to read and send packets asynchronously.

---

## Installation

### Precompiled Binaries (Recommended)
You can download the precompiled binaries directly from the [Releases section](https://github.com/hthug06/Splatted/releases).

### Build from Source
Make sure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

```bash
git clone https://github.com/hthug06/splatted.git
cd splatted
cargo build --release
```

---

## Usage
Go in the folder where your Splatted program is, open a terminal and use the following command:

```bash
./Splatted --address <address> [-p <port> | -i | -b <bot_number> | -r <protocol>]
```
-i is for the info of the server (like in the server list of your Minecraft client)


| Arguments          |  Short   | Default           |                                                                                                             Description                                                                                                             |
|--------------------|:--------:|-------------------|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
| ```--address```    |    -     | **Required**      |                                                                                                      The server IP or hostname                                                                                                      |
| ```--port```       | ```-p``` | ```25565```       |                                                                                                       The target server port                                                                                                        |
| ```--info```       | ```-i``` | ``` false```      |                                                                         See infos about the targetted server like in the server list of a minecraft client                                                                          |
| ```--bot_number``` | ```-b``` | ``` 1```          |                                                                                          The number of bots you want to send to the server                                                                                          |
| ```--protocol```   | ```-r``` | ``` 51 (1.4.7)``` | The protocol version of the minecraft server (click [here](https://minecraft.wiki/w/Protocol_version#Java_Edition_(pre-netty_rewrite)) and scroll down). Found automatically if the target server version is in version 1.4 or more |     

## Roadmap
- [x] Get the server infos like in the multiplayer section of a real client
- [x] Good structure to read / send a packet
- [x] Connect one single bot (pls)
- [x] Send bots to a server and make them disconnect
- [x] Make version 1.2.5 to 1.6.4 work

---

## Other
This project was a way for me to learn rust and the base of the Minecraft protocol.<br>
If you find any bugs or have any suggestions, feel free to open an issue or a pull request or an issue !<br>
Thx [tetram](https://github.com/tetram2674562) ur the goat :)