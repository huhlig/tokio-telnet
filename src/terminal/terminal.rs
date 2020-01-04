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

use crate::consts;
use crate::codec::{DecodeError, EncodeError, TelnetCodec, TelnetFrame};
use crate::terminal::{TerminalError, TerminalInput, TerminalOutput};
use crate::terminal::option::TelnetOption;

use bytes::{BytesMut, BufMut};
use futures::task::{Context, Poll};
use futures::{Sink, Stream, Future};
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;
use crate::terminal::arguments::status::TelnetOptionStatus;
use std::ops::Neg;
use crate::terminal::options::TerminalOption;


///
/// Telnet Network Virtual Terminal Endpoint
///
pub struct NetworkVirtualTerminal<S>
    where
        S: AsyncWrite + AsyncRead,
{
    /// Codec Wrapped Stream
    framed: Framed<S, TelnetCodec>,
    /// Terminal Input Buffer
    buffer: BytesMut,
    /// Current Option Negotiation State
    options: [((bool, NegotiationState), (bool, NegotiationState)); 255],
}

impl<S> NetworkVirtualTerminal<S>
    where
        S: AsyncWrite + AsyncRead,
{
    pub fn new(stream: S) -> NetworkVirtualTerminal<S> {
        NetworkVirtualTerminal {
            framed: Framed::new(stream, TelnetCodec::default()),
            buffer: BytesMut::with_capacity(8 * 4096),
            options: [(NegotationState::No, NegotationState::No); 255],
        }
    }


    /// TODO: Figure out State engine for Negotiation
    fn receive_negotiation(&mut self, action: Action, option: TelnetOption) {
        use self::{State::*, Action::*};
        // https://mudcoders.fandom.com/wiki/Telnet_Option_Negotiation_-_Q_Method
        // @formatter:off
        #[rustfmt::skip]
        match (self.options[option].0, self.options[option].1, action) {
        //  (     Local State,    Remote State, Action) => { Action }
            (              No,               _,     Do) => { self.options[option].0 = Yes; }
            (              No,               _,   Dont) => { /* Ignored */ }
            (               _,              No,   Will) => { self.options[option].1 = Yes; }
            (               _,              No,   Wont) => { }
        //    (Recv,   Will,               _,              No) => { self.options[option].1 = NegotiationState::WantYes }
        //    (Recv,   Wont,               _,              No) => { /* Do Nothing */ }
        //
        }
        // @formatter:on
    }
}

impl<S> Stream for NetworkVirtualTerminal<S>
    where
        S: AsyncRead,
{
    type Item = TerminalOutput;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            let frame = self.framed.poll_next(cx);
            match frame {
                TelnetFrame::Data(ch) => {
                    // TODO: Handle Data Stream
                    self.buffer.put_u8(ch);
                    if self.buffer.len() >= self.buffer.capacity() {
                        let data = Vec::from(self.buffer.as_ref());
                        return Poll::Ready(Some(TerminalOutput::BinaryData(data)));
                    }
                }
                TelnetFrame::AbortOutput => {
                    return Poll::Ready(Some(TerminalOutput::AbortOutput));
                }
                TelnetFrame::AreYouThere => {
                    return Poll::Ready(Some(TerminalOutput::AreYouThere));
                }
                TelnetFrame::Break => {
                    return Poll::Ready(Some(TerminalOutput::Break));
                }
                TelnetFrame::DataMark => {
                    return Poll::Ready(Some(TerminalOutput::DataMark));
                }
                TelnetFrame::EraseCharacter => {
                    return Poll::Ready(Some(TerminalOutput::EraseCharacter));
                }
                TelnetFrame::EraseLine => {
                    return Poll::Ready(Some(TerminalOutput::EraseLine));
                }
                TelnetFrame::GoAhead => {
                    return Poll::Ready(Some(TerminalOutput::GoAhead));
                }
                TelnetFrame::InterruptProcess => {
                    return Poll::Ready(Some(TerminalOutput::InterruptProcess));
                }
                TelnetFrame::NoOperation => {
                    return Poll::Ready(Some(TerminalOutput::NoOperation));
                }
                TelnetFrame::Do(opt) => {
                    let option = TelnetOption::from(opt);
                    if self.options.0.is_none() {
                        self.options.0 = (option, Side::Local, )
                    }
                    self.options[opt]
                }
                TelnetFrame::Dont(opt) => {
                    let option = TelnetOption::from(opt);
                }
                TelnetFrame::Will(opt) => {
                    let option = TelnetOption::from(opt);
                }
                TelnetFrame::Wont(opt) => {
                    let option = TelnetOption::from(opt);
                }
                TelnetFrame::Subnegotiate(opt, arguments) => {
                    let option = TelnetOption::from(opt);
                }
            }
        }
        frame // TODO: Remove this
    }
}

impl<S> Sink<TerminalInput> for NetworkVirtualTerminal<S>
    where
        S: AsyncWrite,
{
    type Error = EncodeError;

    fn poll_ready(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.framed.poll_ready(cx)
    }

    fn start_send(
        self: Pin<&mut Self>,
        item: TerminalInput,
    ) -> Result<(), Self::Error> {
        match item {
            TerminalInput::NoOperation => {
                self.framed.start_send(TelnetFrame::NoOperation)
            },
            TerminalInput::DataMark => {
                self.framed.start_send(TelnetFrame::DataMark)
            },
            TerminalInput::Break => {
                self.framed.start_send(TelnetFrame::Break)
            },
            TerminalInput::InterruptProcess => {
                self.framed.start_send(TelnetFrame::InterruptProcess),
            }
            TerminalInput::AbortOutput => {
                self.framed.start_send(TelnetFrame::AbortOutput)
            },
            TerminalInput::AreYouThere => {
                self.framed.start_send(TelnetFrame::AreYouThere)
            },
            TerminalInput::EraseCharacter => {
                self.framed.start_send(TelnetFrame::EraseCharacter)
            },
            TerminalInput::EraseLine => {
                self.framed.start_send(TelnetFrame::EraseLine)
            },
            TerminalInput::GoAhead => {
                self.framed.start_send(TelnetFrame::GoAhead)
            },
            TerminalInput::AsciiData(data) => {
                /// TODO: Handle Compatability settings
                for byte in data.as_bytes() {
                    self.framed.start_send(TelnetFrame::Data(*byte))
                }
            }
            TerminalInput::BinaryData(data) => {
                /// TODO: Handle Compatability settings
                for byte in data.as_bytes() {
                    self.framed.start_send(TelnetFrame::Data(*byte))
                }
            }
            TerminalInput::EnableLocalOption(TelnetOption) => {},
            TerminalInput::DisableLocalOption(TelnetOption) => {},
            TerminalInput::EnableRemoteOption(TelnetOption) => {},
            TerminalInput::DisableRemoteOption(TelnetOption) => {},
        }
        Ok(())
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.framed.poll_flush(cx)
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.framed.poll_close(cx)
    }
}

pub struct Compatibility {
    send_unicode_while_normal: bool,
    recv_unicode_while_normal: bool,
}
