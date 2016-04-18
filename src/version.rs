// Copyright © 2016 the Bolt ϟ developers
//
// Bolt ϟ is free software; You can redistribute it and/or modify it under the terms of either:
// - the MIT License as published by the Massachusetts Institute of Technology.
// - the Apache License version 2 as published by the Apache Software Foundation.
//
// You don't have to do anything special to choose one license or the other and you don’t have to notify anyone which license you are using.
//
// Bolt ϟ is distributed in the hope that it will be useful, but without any warranty;
// without even the implied warranty of merchantability or fitness for a particular purpose.
// See your chosen license for more details.
//
// You should have received a copy of all licenses along with Bolt ϟ.
// If not, see <https://github.com/minora-oss/bolt/blob/master/license/>.

use std::fmt;

use self::ProtocolVersion::{Http09, Http10, Http11, Http20, Spdy10, Spdy20, Spdy30, Spdy31, Spdy40, Quic10};

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash, Debug)]
pub enum ProtocolVersion {
    /// `HTTP/0.9`
    Http09,
    /// `HTTP/1.0`
    Http10,
    /// `HTTP/1.1`
    Http11,
    /// `HTTP/2.0`
    Http20,
    /// `SPDY/1.0`
    Spdy10,
    /// `SPDY/2.0`
    Spdy20,
    /// `SPDY/3.0`
    Spdy30,
    /// `SPDY/3.1`
    Spdy31,
    /// `SPDY/4.0`
    Spdy40,
    /// `QUIC/1.0`
    Quic10
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Http09 => "HTTP/0.9",
            Http10 => "HTTP/1.0",
            Http11 => "HTTP/1.1",
            Http20 => "HTTP/2.0",
            Spdy10 => "SPDY/1.0",
            Spdy20 => "SPDY/2.0",
            Spdy30 => "SPDY/3.0",
            Spdy31 => "SPDY/3.1",
            Spdy40 => "SPDY/4.0",
            Quic10 => "QUIC/1.0",
        })
    }
}
