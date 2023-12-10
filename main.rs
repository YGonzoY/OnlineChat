use std::net::TcpListener;
use std::result;
use std::io::Write;
use std::fmt;

type Result<T> = result::Result<T, ()>;
const SAFEMODE: bool = false;

struct Sensitive<T>(T);



impl<T: fmt::Display> fmt::Display for Sensitive<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let Self(inner) = self;
        if SAFEMODE
        {
            writeln!(f, "Redacted")
        }
        else
        {
            inner.fmt(f)
        }
    }
}

fn main() -> Result<()> 
{
    let address = "0.0.0.0:6969";
    let listener = TcpListener::bind(address).map_err(|err| 
        {
            eprintln!("ERROR: can not bind {address} : {}", Sensitive(err))
        })?;
    println!("Listening to {}", Sensitive(address));
    for stream in listener.incoming()
    {
        match stream
        {
            Ok(mut stream) => {let _ = writeln!(stream, "Hello").map_err(|err| 
            {
                eprintln!("ERROR: can not write message {err}")
            });},
            Err(err) => {eprintln!("ERROR : {err}");}
        }
    }
    Ok(())
}
