The HAM is an offline, peer-to-peer encrypted messaging system built in Rust using libp2p. It enables nearby devices to communicate without internet access, cellular data, or central servers.
Development is ongoing and many things to add in obvio and the core messaging part works as of now on the same WIFI connection but thinking of making it more secure using maybe bluetooth mesh tech maybe. 

thinking to somehow use sim cards without owners i.e. that havent been registered yet

would love any insights if yall have 

project was made by taking inspiration from the movie "Dhurandhar" + "Bit-Chat" (The new venture of ex-Twitter CEO Jack) + my 3rd sem INCS mini project. 


the project works barely in this infancy stage on Ubuntu but needs testing on windows. 

To Run:

Install Rust first 

```bash
git clone https://github.com/shreyas-sridhar/HAM-Chat.git
cd HAM-Chat
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Test Peer-to-Peer Messaging

```bash
# Open two terminals on the same Wi-Fi network
cargo run
```

Type a message in one terminal to see it appear in the other.
No internet connection is required. But for now your system(s) needs to be connected to the same WIFI 

---
