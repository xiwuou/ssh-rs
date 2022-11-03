//! Dependencies
//! ```toml
//! ssh-rs = "0.2.1"
//! ```
//!
//! ## Connection method：
//!
//! ### 1. Password:
//! ```no_run
//! use ssh_rs::{Session, ssh};
//!
//! let mut session: Session = ssh::create_session();
//! session.set_user_and_password("user", "password");
//! session.connect("ip:port").unwrap();
//! ```
//!
//! ### 2. Public key:
//! #### Currently, only `RSA-PKCS#1-PEM` type encrypted files with the encryption format `-----BEGIN RSA PRIVATE KEY-----` are supported.
//!
//! #### 1. Use key file path：
//! ```no_run
//! use ssh_rs::{Session, ssh};
//! use ssh_rs::key_pair::KeyPairType;
//!
//! let mut session: Session = ssh::create_session();
//! // pem format key path -> /xxx/xxx/id_rsa
//! // KeyPairType::SshRsa -> Rsa type algorithm, currently only supports rsa.
//! session.set_user_and_key_pair_path("user", "pem format key path", KeyPairType::SshRsa).unwrap();
//! session.connect("ip:port").unwrap();
//! ```
//!
//! #### 2. Use key string：
//! ```no_run
//! use ssh_rs::{Session, ssh};
//! use ssh_rs::key_pair::KeyPairType;
//!
//! let mut session: Session = ssh::create_session();
//! // pem format key string:
//! //      -----BEGIN RSA PRIVATE KEY-----
//! //          xxxxxxxxxxxxxxxxxxxxx
//! //      -----END RSA PRIVATE KEY-----
//! // KeyPairType::SshRsa -> Rsa type algorithm, currently only supports rsa.
//! session.set_user_and_key_pair("user", "pem format key string", KeyPairType::SshRsa).unwrap();
//! session.connect("ip:port").unwrap();
//! ```
//!
//! ## Enable global logging：
//!
//! ```no_run
//! use ssh_rs::{Session, ssh};
//!
//! // is_enable_log Whether to enable global logging
//! // The default is false(Do not enable)
//! // Can be set as true (enable)
//! ssh::is_enable_log(true);
//!
//! let mut session: Session = ssh::create_session();
//! session.set_user_and_password("user", "password");
//! session.connect("ip:port").unwrap();
//! ```
//!
//!
//! ## Set timeout：
//!
//! ```no_run
//! use ssh_rs::{Session, ssh};
//!
//! let mut session: Session = ssh::create_session();
//! // set_timeout
//! // The unit is seconds
//! // The default timeout is 30 seconds
//! session.set_timeout(15);
//! session.set_user_and_password("user", "password");
//! session.connect("ip:port").unwrap();
//! ```
//!
//!
//! ## How to use：
//!
//! ### Currently only supports exec shell scp these three functions.
//!
//! ### 1. exec
//!
//! ```no_run
//! use ssh_rs::{ChannelExec, Session, ssh};
//!
//! let mut session: Session = ssh::create_session();
//! // Usage 1
//! let exec: ChannelExec = session.open_exec().unwrap();
//! let vec: Vec<u8> = exec.send_command("ls -all").unwrap();
//! println!("{}", String::from_utf8(vec).unwrap());
//! // Usage 2
//! let channel = session.open_channel().unwrap();
//! let exec = channel.open_exec().unwrap();
//! let vec: Vec<u8> = exec.send_command("ls -all").unwrap();
//! println!("{}", String::from_utf8(vec).unwrap());
//! // Close session.
//! session.close().unwrap();
//! ```
//!
//! ### 2. shell
//!
//! ```no_run
//! use std::thread::sleep;
//! use std::time::Duration;
//! use ssh_rs::{Channel, ChannelShell, Session, ssh};
//!
//! let mut session: Session = ssh::create_session();
//! // Usage 1
//! let mut shell: ChannelShell = session.open_shell().unwrap();
//! run_shell(&mut shell);
//! // Usage 2
//! let channel: Channel = session.open_channel().unwrap();
//! let mut shell = channel.open_shell().unwrap();
//! run_shell(&mut shell);
//! // Close channel.
//! shell.close().unwrap();
//! // Close session.
//! session.close().unwrap();
//!
//! fn run_shell(shell: &mut ChannelShell) {
//!     sleep(Duration::from_millis(500));
//!     let vec = shell.read().unwrap();
//!     println!("{}", String::from_utf8(vec).unwrap());
//!
//!     shell.write(b"ls -all\n").unwrap();
//!
//!     sleep(Duration::from_millis(500));
//!
//!     let vec = shell.read().unwrap();
//!     println!("{}", String::from_utf8(vec).unwrap());
//! }
//! ```
//!
//! ### 3. scp
//!
//! ```no_run
//! use ssh_rs::{Channel, ChannelScp, Session, ssh};
//!
//! let mut session: Session = ssh::create_session();
//! // Usage 1
//! let scp: ChannelScp = session.open_scp().unwrap();
//! scp.upload("local path", "remote path").unwrap();
//!
//! let scp: ChannelScp = session.open_scp().unwrap();
//! scp.download("local path", "remote path").unwrap();
//!
//! // Usage 2
//! let channel: Channel = session.open_channel().unwrap();
//! let scp: ChannelScp = channel.open_scp().unwrap();
//! scp.upload("local path", "remote path").unwrap();
//!
//! let channel: Channel = session.open_channel().unwrap();
//! let scp: ChannelScp = channel.open_scp().unwrap();
//! scp.download("local path", "remote path").unwrap();
//!
//! session.close().unwrap();
//!
//! ```

mod client;
mod client_r;
mod client_w;
mod session;
mod session_auth;
mod channel;
mod kex;
mod channel_shell;
mod channel_exec;
mod channel_scp;
mod channel_scp_d;
mod channel_scp_u;
mod config;
mod util;
mod window_size;
mod slog;
mod constant;
mod data;
mod packet;
mod algorithm;
mod user_info;
mod timeout;

pub mod error;
pub(crate) mod h;
pub mod key_pair;

pub use channel::Channel;
pub use channel_exec::ChannelExec;
pub use channel_scp::ChannelScp;
pub use channel_shell::ChannelShell;
pub use session::Session;
pub use user_info::UserInfo;

use crate::error::{SshError, SshResult};

pub mod ssh {
    use crate::slog::Slog;
    use crate::Session;

    pub fn create_session() -> Session {
        Session {
            timeout_sec: 30,
            user_info: None,
            client: None,
            client_channel_no: 0,
        }
    }

    pub fn is_enable_log(b: bool) {
        if b {
            Slog::default()
        }
    }
}
