use crate::Result;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use tokio::io::{AsyncReadExt, AsyncWriteExt, Error, ErrorKind};
use tokio::net::TcpStream;

pub(crate) mod client;

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum Command {
    SwitchOn,
    SwitchOff,
    GetState,
}

macro_rules! create_recv_send_functions {
    ($t:ty) => {
        paste::paste! {
            pub(crate) async fn [<send_ $t>](data: $t, stream: &mut TcpStream) -> Result<()> {
                let data_bytes = data.to_be_bytes();
                stream.write_all(&data_bytes).await?;
                Ok(())
            }

            pub(crate) async fn [<recv_ $t>](stream: &mut TcpStream) -> Result<$t> {
                let mut buf = [0; std::mem::size_of::<$t>()];
                stream.read_exact(&mut buf).await?;
                Ok($t::from_be_bytes(buf))
            }
        }
    };
}

create_recv_send_functions!(u8);
create_recv_send_functions!(u32);

pub(crate) async fn send_command(cmd: Command, stream: &mut TcpStream) -> Result<()> {
    send_u8(cmd.into(), stream).await
}

pub(crate) async fn recv_command(stream: &mut TcpStream) -> Result<Command> {
    Command::try_from(recv_u8(stream).await?)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e).into())
}

pub(crate) async fn send_bool(b: bool, stream: &mut TcpStream) -> Result<()> {
    send_u8(b as u8, stream).await
}

pub(crate) async fn recv_bool(stream: &mut TcpStream) -> Result<bool> {
    let data = recv_u8(stream).await?;
    let (t, f) = (true as u8, false as u8);
    match data {
        x if x == t => Ok(true),
        x if x == f => Ok(false),
        x => Err(Error::new(
            ErrorKind::InvalidData,
            format!("{} cannot represent bool", x),
        )
        .into()),
    }
}
