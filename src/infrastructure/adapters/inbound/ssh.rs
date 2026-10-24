use russh::keys::ssh_encoding::Encode;
use russh::keys::{Certificate, *};
use russh::server::{Msg, Session};
use russh::*;

use crate::application::ports::InboundPort;
use crate::application::dto::AppCommandRequest;

#[derive(Clone)]
pub struct SSHInbound {
    core: std::sync::Arc<crate::application::app::Core>
}

pub enum AccessLevel {
    Unauthenticated,
    Authenticated
}

pub struct ClientSession {
    current_access_level: AccessLevel,
    current_buffer: Vec<u8>,
    history: Vec<String>,
    core: std::sync::Arc<crate::application::app::Core>
}

impl ClientSession {
    pub fn new(core: std::sync::Arc<crate::application::app::Core>) -> ClientSession {
        ClientSession {
            current_access_level: AccessLevel::Unauthenticated,
            current_buffer: vec![],
            history: vec![],
            core
        }
    }
}

impl SSHInbound {
    pub fn new(core: std::sync::Arc<crate::application::app::Core>) -> SSHInbound {
        SSHInbound {
            core
        }
    }
}

impl server::Server for SSHInbound {
    type Handler = ClientSession;

    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> ClientSession {
        ClientSession::new(self.core.clone())
    }
    fn handle_session_error(&mut self, _error: <Self::Handler as russh::server::Handler>::Error) {
        eprintln!("Session error: {_error:#?}");
    }
}

impl server::Handler for ClientSession {
    type Error = russh::Error;

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        reply: server::ChannelOpenHandle,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        reply.accept().await;

        Ok(())
    }

    async fn auth_publickey(
        &mut self,
        _: &str,
        _key: &ssh_key::PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        Ok(server::Auth::Accept)
    }

    async fn auth_openssh_certificate(
        &mut self,
        _user: &str,
        _certificate: &Certificate,
    ) -> Result<server::Auth, Self::Error> {
        Ok(server::Auth::Accept)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> { 
        for &byte in data {
            match byte {
                3 => {
                    return Err(russh::Error::Disconnect);
                }
                // Handle Enter key: Carriage Return (\r) or Line Feed (\n)
                b'\r' | b'\n' => {
                    // 1. Process the accumulated command/text 
                    if !self.current_buffer.is_empty() {
                        let echo = CryptoVec::from_slice(b"\r\n");
                        session.data(channel, echo.encode_bytes()?)?;

                        let query_ : String = String::from_utf8(self.current_buffer.to_vec()).unwrap();

                        if query_.to_lowercase() == "exit" {
                            return Err(russh::Error::Disconnect)
                        }

                        match self.core.command(AppCommandRequest::new(query_.clone())).await {
                            Ok(res) => {
                                let mut pretty_json = serde_json::to_string_pretty(&res.result).unwrap();
                                pretty_json = pretty_json.replace('\n', "\r\n");
                                
                                let echo = CryptoVec::from_slice(&pretty_json.into_bytes());
                                session.data(channel, echo.encode_bytes()?)?;  
                            },

                            Err(err) => {
                                let echo = CryptoVec::from_slice(&format!("{}", err).into_bytes());
                                session.data(channel, echo.encode_bytes()?)?;               
                            }
                        }

                        self.history.push(query_);
                    }

                    // 2. Clear the buffer for the next input
                    self.current_buffer.clear();

                    // 3. Echo a newline back to the client terminal
                    let echo = CryptoVec::from_slice(b"\r\n(terminal)# ");
                    session.data(channel, echo.encode_bytes()?)?;
                }
                // Handle Backspace (ASCII 127 or 8)
                127 | 8 => {
                    if self.current_buffer.pop().is_some() {
                        // Move cursor back, overwrite char with space, move back again
                        let echo = CryptoVec::from_slice(b"\x08 \x08");
                        session.data(channel, echo.encode_bytes()?)?;
                    }
                }
                // Handle normal printable ASCII characters
                32..=126 => {
                    self.current_buffer.push(byte);

                    // Echo the character back to the client so they see what they type
                    let echo = CryptoVec::from_slice(&[byte]);
                    session.data(channel, echo.encode_bytes()?)?;
                }
                _ => {
                    // Ignore control characters or unhandled escape sequences
                }
            }
        }

        Ok(())
    }
}
