use russh::keys::ssh_encoding::Encode;
use russh::keys::*;
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
    current_buffer: Vec<u8>,
    history: Vec<String>,
    core: std::sync::Arc<crate::application::app::Core>
}

impl ClientSession {
    pub fn new(core: std::sync::Arc<crate::application::app::Core>) -> ClientSession {
        ClientSession {
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
        _user: &str,
        _key: &ssh_key::PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        let key_path = format!("id_ed25519.{}.pub", _user);

        match russh::keys::load_public_key(std::path::Path::new(&key_path)) {
            Ok(allowed_key) => {
                if _key == &allowed_key {
                    Ok(russh::server::Auth::Accept)
                } else {
                    Ok(russh::server::Auth::Reject {
                        partial_success: false,
                        proceed_with_methods: Some(MethodSet::server_supported())
                    })
                }
            }
            Err(e) => {
                tracing::error!("Failed to load server-side public key file: {}", e);
                Ok(russh::server::Auth::Reject {
                    partial_success: false,
                    proceed_with_methods: Some(MethodSet::server_supported())
                })
            }
        }
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
                b'\r' | b'\n' => {
                    if !self.current_buffer.is_empty() {
                        let echo = CryptoVec::from_slice(b"\r\n");
                        session.data(channel, echo.encode_bytes()?)?;

                        let query_ : String = match String::from_utf8(self.current_buffer.to_vec()) {
                            Ok(str_) => str_,
                            Err(err) => {
                                tracing::error!("Cannot parse buffer: {}", err);
                                return Err(russh::Error::Disconnect);
                            }
                        };

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

                    self.current_buffer.clear();

                    let echo = CryptoVec::from_slice(b"\r\n(terminal)# ");
                    session.data(channel, echo.encode_bytes()?)?;
                }
                127 | 8 => {
                    if self.current_buffer.pop().is_some() {
                        let echo = CryptoVec::from_slice(b"\x08 \x08");
                        session.data(channel, echo.encode_bytes()?)?;
                    }
                }
                32..=126 => {
                    self.current_buffer.push(byte);

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
