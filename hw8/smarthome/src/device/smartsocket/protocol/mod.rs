use crate::Result;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::{Error, ErrorKind, Read, Write};

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
            pub(crate) fn [<send_ $t>]<W: Write>(data: $t, mut w: W) -> Result<()> {
                let data_bytes = data.to_be_bytes();
                w.write_all(&data_bytes)?;
                Ok(())
            }

            pub(crate) fn [<recv_ $t>]<R: Read>(mut r: R) -> Result<$t> {
                let mut buf = [0; std::mem::size_of::<$t>()];
                r.read_exact(&mut buf)?;
                Ok($t::from_be_bytes(buf))
            }
        }
    };
}

create_recv_send_functions!(u8);
create_recv_send_functions!(u32);

pub(crate) fn send_command<W: Write>(c: Command, w: W) -> Result<()> {
    send_u8(c.into(), w)
}

pub(crate) fn recv_command<R: Read>(r: R) -> Result<Command> {
    Command::try_from(recv_u8(r)?).map_err(|e| Error::new(ErrorKind::InvalidData, e).into())
}

pub(crate) fn send_bool<W: Write>(b: bool, mut w: W) -> Result<()> {
    send_u8(b as u8, &mut w)
}

pub(crate) fn recv_bool<R: Read>(mut r: R) -> Result<bool> {
    let data = recv_u8(&mut r)?;
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
