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

use super::consts;
use super::error::DecodeError;
use super::error::EncodeError;
use super::frame::TelnetFrame;

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

///
///
///
pub struct TelnetCodec {
    decoder_buffer: BytesMut,
    decoder_state: DecoderState,
}

#[derive(Clone, Copy, Debug)]
enum DecoderState {
    /// Normal Data
    NormalData,
    /// Received IAC, Next byte is Command
    InterpretAsCommand,
    /// Received Do Command, Next Byte is option
    NegotiateDo,
    /// Received Dont Command, Next Byte is option
    NegotiateDont,
    /// Received Will Command, Next Byte is option
    NegotiateWill,
    /// Received Wont Command, Next Byte is option
    NegotiateWont,
    /// Received Subnegotiate Command, Next Byte is option
    Subnegotiate,
    /// Received Subnegotiate Option, Next Bytes are arguments
    SubnegotiateArgument(u8),
    /// Received IAC during Subnegotiation, Next Byte is command
    SubnegotiateArgumentIAC(u8),
}

impl Decoder for TelnetCodec {
    type Item = TelnetFrame;
    type Error = DecodeError;

    ///
    /// Decode incoming data, then take the first Frame
    ///
    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        //
        // This decode engine attempts to parse a telnet data stream with the following
        // restrictions.
        // * IAC commands cannot be interrupted.
        // * IAC Commands inside subnegotiation beyond IAC IAC and IAC SE will return error.
        // *
        //
        //
        while src.remaining() > 0 {
            let byte = src.get_u8();
            match (self.decoder_state, byte) {
                (DecoderState::NormalData, consts::IAC) => {
                    self.decoder_state = DecoderState::InterpretAsCommand;
                }
                (DecoderState::NormalData, _) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::Data(byte)));
                }
                (DecoderState::InterpretAsCommand, consts::NOP) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::NoOperation));
                }
                (DecoderState::InterpretAsCommand, consts::DM) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::DataMark));
                }
                (DecoderState::InterpretAsCommand, consts::BRK) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::Break));
                }
                (DecoderState::InterpretAsCommand, consts::IP) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::InterruptProcess));
                }
                (DecoderState::InterpretAsCommand, consts::AO) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::AbortOutput));
                }
                (DecoderState::InterpretAsCommand, consts::AYT) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::AreYouThere));
                }
                (DecoderState::InterpretAsCommand, consts::EC) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::EraseCharacter));
                }
                (DecoderState::InterpretAsCommand, consts::EL) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::EraseLine));
                }
                (DecoderState::InterpretAsCommand, consts::GA) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::GoAhead));
                }
                (DecoderState::InterpretAsCommand, consts::IAC) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::Data(consts::IAC)));
                }
                (DecoderState::InterpretAsCommand, consts::DO) => {
                    self.decoder_state = DecoderState::NegotiateDo;
                }
                (DecoderState::InterpretAsCommand, consts::DONT) => {
                    self.decoder_state = DecoderState::NegotiateDont;
                }
                (DecoderState::InterpretAsCommand, consts::WILL) => {
                    self.decoder_state = DecoderState::NegotiateWill;
                }
                (DecoderState::InterpretAsCommand, consts::WONT) => {
                    self.decoder_state = DecoderState::NegotiateWont;
                }
                (DecoderState::InterpretAsCommand, consts::SB) => {
                    self.decoder_state = DecoderState::Subnegotiate;
                }
                (DecoderState::InterpretAsCommand, _) => {
                    // Return to normal data and return a Decode Error
                    self.decoder_state = DecoderState::NormalData;
                    return Err(DecodeError::UnknownCommand(byte));
                }
                (DecoderState::NegotiateDo, _) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::Do(byte)));
                }
                (DecoderState::NegotiateDont, _) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::Dont(byte)));
                }
                (DecoderState::NegotiateWill, _) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::Will(byte)));
                }
                (DecoderState::NegotiateWont, _) => {
                    self.decoder_state = DecoderState::NormalData;
                    return Ok(Some(TelnetFrame::Wont(byte)));
                }
                (DecoderState::Subnegotiate, _) => {
                    self.decoder_state =
                        DecoderState::SubnegotiateArgument(byte);
                }
                (DecoderState::SubnegotiateArgument(option), consts::IAC) => {
                    self.decoder_state =
                        DecoderState::SubnegotiateArgumentIAC(option);
                }
                (DecoderState::SubnegotiateArgument(_option), _) => {
                    self.decoder_buffer.put_u8(byte);
                }
                (
                    DecoderState::SubnegotiateArgumentIAC(option),
                    consts::IAC,
                ) => {
                    self.decoder_state =
                        DecoderState::SubnegotiateArgument(option);
                    self.decoder_buffer.put_u8(consts::IAC);
                }
                (DecoderState::SubnegotiateArgumentIAC(option), consts::SE) => {
                    self.decoder_state = DecoderState::NormalData;
                    let buffer = Vec::from(self.decoder_buffer.as_ref());
                    self.decoder_buffer.clear();
                    return Ok(Some(TelnetFrame::Subnegotiate(option, buffer)));
                }
                (DecoderState::SubnegotiateArgumentIAC(_), _) => {
                    // Decoding error, drop back to normal
                    self.decoder_state = DecoderState::NormalData;
                    return Err(DecodeError::UnknownCommand(byte));
                }
            }
        }
        Ok(None)
    }
}

impl Encoder for TelnetCodec {
    type Item = TelnetFrame;
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: Self::Item,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        match item {
            TelnetFrame::Data(ch) => {
                dst.reserve(2);
                if ch == consts::IAC {
                    dst.put_u8(consts::IAC);
                }
                dst.put_u8(ch);
            }
            TelnetFrame::NoOperation => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::NOP);
            }
            TelnetFrame::DataMark => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::DM);
            }
            TelnetFrame::Break => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::BRK);
            }
            TelnetFrame::InterruptProcess => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::IP);
            }
            TelnetFrame::AbortOutput => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::AO);
            }
            TelnetFrame::AreYouThere => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::AYT);
            }
            TelnetFrame::EraseCharacter => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::EC);
            }
            TelnetFrame::EraseLine => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::EL);
            }
            TelnetFrame::GoAhead => {
                dst.reserve(2);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::GA);
            }
            TelnetFrame::Do(option) => {
                dst.reserve(3);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::DO);
                dst.put_u8(option);
            }
            TelnetFrame::Dont(option) => {
                dst.reserve(3);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::DONT);
                dst.put_u8(option);
            }
            TelnetFrame::Will(option) => {
                dst.reserve(3);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::WILL);
                dst.put_u8(option);
            }
            TelnetFrame::Wont(option) => {
                dst.reserve(3);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::WONT);
                dst.put_u8(option);
            }
            TelnetFrame::Subnegotiate(option, arguments) => {
                dst.reserve(5 + arguments.len());
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::SB);
                dst.put_u8(option);
                dst.put_slice(&arguments);
                dst.put_u8(consts::IAC);
                dst.put_u8(consts::SE);
            }
        }
        Ok(())
    }
}

impl Default for TelnetCodec {
    fn default() -> TelnetCodec {
        TelnetCodec {
            decoder_buffer: BytesMut::new(),
            decoder_state: DecoderState::NormalData,
        }
    }
}
