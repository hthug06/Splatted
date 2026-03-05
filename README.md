# Splatted 🦑
Splatted is a lightweight command-line utility written in Rust for interacting with Minecraft servers. It focuses on low-level packet handling and asynchronous communication to query server metadata.

## How to use it 
First of all, make sure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

### installation
```bash
git clone https://github.com/your-username/splatted.git
cd splatted
cargo build --release
```

### Usage
Go in the folder where your Splatted program is, open a terminal and use the following command:

```bash
./Splastted --adress <adress> [-p <port> | -i]
```
-i is for the info of the server (like in the server list of your minecraft client)


| Arguments       |  Short   | Default      |                    Description                     |
|-----------------|:--------:|--------------|:--------------------------------------------------:|
| ```--address``` |    -     | **Required** |             The server IP or hostname              |
| ```--port```    | ```-p``` | ```25565```  |               The target server port               |
| ```--info```    | ```-i``` | ``` false``` | See like in the server list of a minecraft  client |

## Roadmap
- [ ] Connect 1 single bot (pls)
- [ ] Good structure to read / send packet
- [ ] Send bots to a server and make them disconnect

## Other
Thx [tetram](https://github.com/tetram2674562) ur the goat :)