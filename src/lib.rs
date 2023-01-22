//! # olc-datafile-rust
//! ---
//! [![Downloads](https://img.shields.io/crates/d/olc-datafile-rust)](https://crates.io/crates/olc-datafile-rust)
//! [![Version](https://img.shields.io/crates/v/olc-datafile-rust)](https://crates.io/crates/olc-datafile-rust)
//! ---
//!
//! A rust library for reading and writing olc::DataFile files.
//! The olc::DataFile format is a simple text file format for storing data in a human readable and editable format. In the
//! words of the original author, it's "great for serializing and deserializing data, i.e. Saving Things!".
//!
//! The original C++ implementation can be found [in the OLC::PixelGameEngine repository](https://github.com/OneLoneCoder/olcPixelGameEngine/blob/master/utilities/olcUTIL_DataFile.h).
//! An explanation and full walkthrough of it can be found in [this video](https://www.youtube.com/watch?v=jlS1Y2-yKV0)
//! on the OneLoneCoder YouTube channel.
//!
//! In case you just want to compare the code to the original C++ version, you'll find
//!
//! * The Datafile class in [src/datafile.rs](src/datafile.rs)
//! * The parsing/reading in [src/processor/reader.rs](src/processor/reader.rs)
//! * The serialization/writing in [src/processor/writer.rs](src/processor/writer.rs)
//!
//! ## Usage
//!
//! ```no_run
//! use olc_datafile_rust::Datafile;
//!
//! let mut datafile = Datafile::new(Some(','), Some(" "));
//!
//! let some_node = datafile.get("some_node");
//! some_node.get("name").set_string("Javid", 0);
//! some_node.get("age").set_integer(24, 0);
//! some_node.get("height").set_real(1.88, 0);
//!
//! let code = some_node.get("code");
//! code.set_string("c++", 0);
//! code.set_string("vhdl", 1);
//! code.set_string("lua", 2);
//!
//! let pc = some_node.get("pc");
//! pc.get("processor").set_string("intel", 0);
//! pc.get("ram").set_integer(32, 0);
//!
//! datafile
//!     .write("./datafile.txt")
//!     .expect("Failed to write datafile");
//!
//! let mut datafile = Datafile::new(Some(','), Some(" "));
//!
//! datafile
//!     .read("./datafile.txt")
//!     .expect("Failed to read datafile");
//!
//! println!("{:?}", datafile.get("some_node"));
//! ```
//!
//! ## Goals
//!
//! The goal was to implement it as closely as possible to the original, while still leveraging the features of Rust.
//! Even with that goal some minor differences exist:
//!
//! - API differences:
//!     * Reading and writing are handled through separate structs. The APIs are accessible through the `Datafile` itself.
//!       The
//!       structs themselves can be used directly if desired.
//!     * Reading and writing also don't happen statically. Either an instance of `Datafile` or `Writer`/`Reader`
//!       must be created.
//!
//! - Implementation differences:
//!     * The original parser was implemented using a stack based approach. Whilst it's a good solution, this
//!       implementation uses a recursion based approach. Not only was it easier to implement, but it also eliminated the
//!       need to keep track of the references as it was done in the original implementation.
//!     * Some internal methods were added to make the code a bit more readable. These methods are not part of the public
//!       API, and comparing both codebases should still be trivial.
//!     * The original implementation was done in a single header file. I opted for a multi-file approach.
//!
//! # License (OLC-3)
//!
//! Copyright 2018-2023 OneLoneCoder.com
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions
//! are met:
//!
//! 1. Redistributions or derivations of source code must retain the above
//!    copyright notice, this list of conditions and the following disclaimer.
//!
//! 2. Redistributions or derivative works in binary form must reproduce
//!    the above copyright notice. This list of conditions and the following
//!    disclaimer must be reproduced in the documentation and/or other
//!    materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived
//!    from this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
//! "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
//! LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
//! A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
//! HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//! SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
//! LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
//! DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
//! THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
//! (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#[rustfmt::skip]
pub use {
    datafile::Datafile,
    processor::reader::Reader,
    processor::writer::Writer,
};

/// The `datafile` module contains the `Datafile` struct and its methods.
pub mod datafile;

/// The `processor` module contains the `Reader` and `Writer` structs and their methods.
/// These structs are used to read and write datafiles, respectively. In theory, accessing
/// these structs directly is not necessary, as the `Datafile` struct provides a more
/// convenient interface.
pub mod processor;

mod lexical;
