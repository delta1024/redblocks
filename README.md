
# About
Redblocks is a library inspired by xmobar for creating your own status blocks that writes to XROOTNAME. Primaraly designed with dwm in mind. 

# Dependencies 
- xsetroot
# Usage Requirments
Using redblock is intended to be simple, baring creating custom modules; if this is not the case I would consider that a bug and would engorage you to raise the issue as such.

The one caviate to the aformentioned principle is a basic understanding of rust is required to setup and configure your statusbar. You can paruse the [reference](https://doc.rust-lang.org/reference/introduction.html) for any concepts you don't understand (baring anyghing specific to redblocks). For a more compleate introduction to the language I would encorage you to check out [The Book](https://doc.rust-lang.org/book/) or get started with the [learning guilde](https://www.rust-lang.org/learn). a great place to start learing is [here](https://www.rust-lang.org/learn); if you need help installing Rust please see the [installation guide](https://www.rust-lang.org/tools/install).

# Setup
To use redblocks add the following to your Cargo.toml.

	[dependencies]
	redblocks = 0.2.31

for configuratoin details please see the crate [documentation](https://docs.rs/redblocks/).
