use std::net::{TcpListener, TcpStream};
use std::result;
use std::io::Write;
use std::fmt;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

type Result<T> = result::Result<T, ()>;
const SAFEMODE: bool = false;

struct Sensitive<T>(T);

enum Message
{
    ClientConnected,
    ClientDisconnected,
    NewMessage,
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

fn client(mut stream: TcpStream, _messages: Sender<Message>) -> Result<()>
{
    message.send(Message::ClientConnected).map_err(|err|
    {
        eprintln!("ERROR: could not send message to server thread {err}",
         err = Sensitive(err))
    })?;
    let buffer = Vec::new();
    buffer.resize(64, 0);
    loop
    {
        let n = stream.read(&mut buffer).map_err(|err|
        {
            messages.send(Message::ClientDisconnected);
        })?;
        Message::NewMessage(buffer[0..n]);
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
                let message_sender = message_sender.clone();
                thread::spawn(||{client(stream, message_sender)});
            }
            Err(err) => {eprintln!("ERROR : {err}");}
        }
    }
    Ok(())
}
