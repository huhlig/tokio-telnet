//
// Copyright 2019 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

mod codec;

pub use self::codec::TelnetCodec;

pub mod consts;

mod error;

pub use self::error::DecodeError;
pub use self::error::EncodeError;

mod frame;

pub use self::frame::TelnetFrame;

#[cfg(test)]
mod tests {
    use super::consts;
    use super::{TelnetCodec, TelnetFrame};
    use bytes::BytesMut;
    use tokio_util::codec::{Decoder, Encoder};

    #[test]
    fn telnet_decode() {
        let mut codec = TelnetCodec::default();
        let mut encoded_input = BytesMut::from(&b"Terminated line\r\n"[..]);
        let expected_output = vec![
            TelnetFrame::Data(b'T'),
            TelnetFrame::Data(b'e'),
            TelnetFrame::Data(b'r'),
            TelnetFrame::Data(b'm'),
            TelnetFrame::Data(b'i'),
            TelnetFrame::Data(b'n'),
            TelnetFrame::Data(b'a'),
            TelnetFrame::Data(b't'),
            TelnetFrame::Data(b'e'),
            TelnetFrame::Data(b'd'),
            TelnetFrame::Data(b' '),
            TelnetFrame::Data(b'l'),
            TelnetFrame::Data(b'i'),
            TelnetFrame::Data(b'n'),
            TelnetFrame::Data(b'e'),
            TelnetFrame::Data(b'\r'),
            TelnetFrame::Data(b'\n'),
        ];
        let mut actual_output = Vec::new();
        while let Some(frame) = codec.decode(&mut encoded_input).unwrap() {
            actual_output.push(frame)
        }

        assert_eq!(
            expected_output, actual_output,
            "telnet_decode didn't match"
        );
    }

    #[test]
    fn telnet_encode() {
        let mut codec = TelnetCodec::default();
        let input_frames = vec![
            TelnetFrame::Data(b'R'),
            TelnetFrame::Data(b'a'),
            TelnetFrame::Data(b'w'),
            TelnetFrame::Data(b' '),
            TelnetFrame::Data(b'A'),
            TelnetFrame::Data(b's'),
            TelnetFrame::Data(b'c'),
            TelnetFrame::Data(b'i'),
            TelnetFrame::Data(b'i'),
            TelnetFrame::Data(b' '),
            TelnetFrame::Data(b'D'),
            TelnetFrame::Data(b'a'),
            TelnetFrame::Data(b't'),
            TelnetFrame::Data(b'a'),
            TelnetFrame::Data(b'\r'),
            TelnetFrame::Data(b'\n'),
        ];
        let expected_output = BytesMut::from(&b"Raw Ascii Data\r\n"[..]);
        let mut actual_output = BytesMut::with_capacity(20);

        for input_frame in input_frames {
            codec.encode(input_frame, &mut actual_output).unwrap();
        }

        assert_eq!(
            expected_output, actual_output,
            "telnet_encode didn't match"
        );
    }

    #[test]
    fn decode_iac_activation() {
        let mut codec = TelnetCodec::default();
        let mut encoded_input = BytesMut::from(
            &[
                // Data
                b'L',
                b'o',
                b'g',
                b'i',
                b'n',
                b':',
                consts::CR,
                consts::LF,
                // Command Do Binary
                consts::IAC,
                consts::DO,
                crate::terminal::consts::option::BINARY,
                // Data
                b'P',
                b'a',
                b's',
                b's',
                b'w',
                b'o',
                b'r',
                b'd',
                b':',
                consts::CR,
                consts::LF,
                // Command Will Binary
                consts::IAC,
                consts::WILL,
                crate::terminal::consts::option::BINARY,
                // Data
                b'H',
                b'e',
                b'l',
                b'l',
                b'o',
                b'!',
                consts::CR,
                consts::LF,
            ][..],
        );
        let expected_output = vec![
            // Normal Data
            TelnetFrame::Data(b'L'),
            TelnetFrame::Data(b'o'),
            TelnetFrame::Data(b'g'),
            TelnetFrame::Data(b'i'),
            TelnetFrame::Data(b'n'),
            TelnetFrame::Data(b':'),
            TelnetFrame::Data(consts::CR),
            TelnetFrame::Data(consts::LF),
            // Command Do Binary
            TelnetFrame::Do(crate::terminal::consts::option::BINARY),
            // Data
            TelnetFrame::Data(b'P'),
            TelnetFrame::Data(b'a'),
            TelnetFrame::Data(b's'),
            TelnetFrame::Data(b's'),
            TelnetFrame::Data(b'w'),
            TelnetFrame::Data(b'o'),
            TelnetFrame::Data(b'r'),
            TelnetFrame::Data(b'd'),
            TelnetFrame::Data(b':'),
            TelnetFrame::Data(consts::CR),
            TelnetFrame::Data(consts::LF),
            // Command Will Binary
            TelnetFrame::Will(crate::terminal::consts::option::BINARY),
            // Data
            TelnetFrame::Data(b'H'),
            TelnetFrame::Data(b'e'),
            TelnetFrame::Data(b'l'),
            TelnetFrame::Data(b'l'),
            TelnetFrame::Data(b'o'),
            TelnetFrame::Data(b'!'),
            TelnetFrame::Data(consts::CR),
            TelnetFrame::Data(consts::LF),
        ];
        let mut actual_output = Vec::new();
        while let Some(frame) = codec.decode(&mut encoded_input).unwrap() {
            actual_output.push(frame)
        }

        assert_eq!(expected_output, actual_output);
    }
}
