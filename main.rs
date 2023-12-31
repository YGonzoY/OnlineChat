use std::net::{TcpListener, TcpStream};
use std::result;
use std::io::Write;
use std::io::Read;
use std::fmt;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Arc;
use std::ops::Deref

type Result<T> = result::Result<T, ()>;
const SAFEMODE: bool = false;

struct Sensitive<T>(T);

enum Message
{
    ClientConnected(Arc<TcpStream>),
    ClientDisconnected(Arc<TcpStream>),
    NewMessage(Vec<u8>),
}


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

fn server(_message: Receiver<Message>) -> Result<()>
{
    todo!()
}

fn client(stream: Arc<TcpStream>, messages: Sender<Message>) -> Result<()>
{
    messages.send(Message::ClientConnected(stream.clone())).map_err(|err|
    {
        eprintln!("ERROR: could not send message to server thread {err}")
    })?;
    let mut buffer = Vec::new();
    buffer.resize(64, 0);
    loop
    {
        let n = stream.as_ref().read(&mut buffer).map_err(|err|
            {
                eprintln!("errror could not read message {err}")
                let _ = messages.send(Message::ClientDisconnected(stream.clone()));
            } )?;
        messages.send(Message::NewMessage(buffer[0..n].to_vec())).map_err(|err| 
        {
            eprintln!("ERROR: could not send message to server thread {err}")
        })?;
    }
    todo!()
}

fn main() -> Result<()> 
{
    let address = "0.0.0.0:3434";
    let listener = TcpListener::bind(address).map_err(|err| 
        {
            eprintln!("ERROR: can not bind {address} : {}", Sensitive(err))
        })?;
    println!("Listening to {}", Sensitive(address));
    let (message_sender, message_receiver) = channel();
    thread::spawn(|| server(message_receiver));
    for stream in listener.incoming()
    {
        match stream
        {
            Ok(stream) => 
            { 
                let stream = Arc::new(stream);
                let message_sender = message_sender.clone();
                thread::spawn(||{client(stream, message_sender)});
            }
            Err(err) => {eprintln!("ERROR : {err}");}
        }
    }
    Ok(())
}
