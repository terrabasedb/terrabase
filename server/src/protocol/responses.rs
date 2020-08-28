/*
 * Created on Sat Aug 22 2020
 *
 * This file is a part of TerrabaseDB
 * Copyright (c) 2020, Sayan Nandan <ohsayan at outlook dot com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

//! Primitives for generating Terrapipe compatible responses

pub mod groups {
    //! # Pre-compiled response **groups**
    //! These are pre-compiled response groups and **not** complete responses, that is, this is
    //! to be sent after a `GroupBegin(n)` has been written to the stream. If complete
    //! responses are required, user protocol::responses::fresp
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref OKAY: Vec<u8> = "!1\n0\n".as_bytes().to_owned();
        pub static ref NIL: Vec<u8> = "!1\n1\n".as_bytes().to_owned();
        pub static ref OVERWRITE_ERR: Vec<u8> = "!1\n2\n".as_bytes().to_owned();
        pub static ref ACTION_ERR: Vec<u8> = "!1\n3\n".as_bytes().to_owned();
        pub static ref PACKET_ERR: Vec<u8> = "!1\n4\n".as_bytes().to_owned();
        pub static ref SERVER_ERR: Vec<u8> = "!1\n5\n".as_bytes().to_owned();
        pub static ref OTHER_ERR_EMPTY: Vec<u8> = "!1\n6\n".as_bytes().to_owned();
        pub static ref HEYA: Vec<u8> = "+4\nHEY!\n".as_bytes().to_owned();
        pub static ref UNKNOWN_ACTION: Vec<u8> = "!14\nUnknown action\n".as_bytes().to_owned();
    }
}

pub mod fresp {
    //! # Pre-compiled **responses**
    //! These are pre-compiled **complete** responses. This means that they should
    //! be written off directly to the stream and should **not be preceded by a `GroupBegin(n)`**
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref R_OKAY: Vec<u8> = "#2\n*1\n#2\n&1\n!1\n0\n".as_bytes().to_owned();
        pub static ref R_NIL: Vec<u8> = "#2\n*1\n#2\n&1\n!1\n1\n".as_bytes().to_owned();
        pub static ref R_OVERWRITE_ERR: Vec<u8> = "#2\n*1\n#2\n&1\n!1\n2\n".as_bytes().to_owned();
        pub static ref R_ACTION_ERR: Vec<u8> = "#2\n*1\n#2\n&1\n!1\n3\n".as_bytes().to_owned();
        pub static ref R_PACKET_ERR: Vec<u8> = "#2\n*1\n#2\n&1\n!1\n4\n".as_bytes().to_owned();
        pub static ref R_SERVER_ERR: Vec<u8> = "#2\n*1\n#2\n&1\n!1\n5\n".as_bytes().to_owned();
        pub static ref R_OTHER_ERR_EMPTY: Vec<u8> = "#2\n*1\n#2\n&1\n!1\n6\n".as_bytes().to_owned();
        pub static ref R_HEYA: Vec<u8> = "#2\n*1\n#2\n&1\n+4\nHEY!\n".as_bytes().to_owned();
        pub static ref R_UNKNOWN_ACTION: Vec<u8> = "#2\n*1\n#2\n&1\n!14\nUnknown action\n"
            .as_bytes()
            .to_owned();
    }
}
