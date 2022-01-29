use std::net::TcpStream;
use std::io::{self, BufWriter, BufReader, Read, Write};
use log::info;
use crate::util::SCResult;

const GAME_TYPE: &str = "swc_2022_ostseeschach";

/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait SCClientDelegate {
    // TODO: Add methods
}

/// A configuration that determines whether
/// the reader and/or the writer of a stream
/// should be swapped by stdio to ease debugging.
pub struct DebugMode {
    pub debug_reader: bool,
    pub debug_writer: bool,
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> where D: SCClientDelegate {
    delegate: D,
    debug_mode: DebugMode,
    // TODO: Add game state
}

impl<D> SCClient<D> where D: SCClientDelegate {
    /// Creates a new client using the specified delegate.
    pub fn new(delegate: D, debug_mode: DebugMode) -> Self {
        Self { delegate, debug_mode }
    }
    
    /// Blocks the thread and begins reading XML messages
    /// from the provided address via TCP.
    pub fn run(self, host: &str, port: u16, reservation: Option<&str>) -> SCResult<()> {
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&address)?;
        info!("Connected to {}", address);
        
        {
            let mut writer = BufWriter::new(&stream);
            writer.write("<protocol>".as_bytes())?;
            
            let join_xml = match reservation {
                Some(res) => format!("<joinPrepared reservationCode=\"{}\" />", res),
                None => format!("<join gameType=\"{}\" />", GAME_TYPE)
            };
            info!("Sending join message {}", join_xml);
            writer.write(join_xml.as_bytes())?;
        }
        
        // Begin parsing game messages from the stream.
        // List all combinations of modes explicitly,
        // since they generate different generic instantiations
        // of `run_game`.

        let mode = &self.debug_mode;
        if mode.debug_reader && !mode.debug_writer {
            self.run_game(io::stdin(), BufWriter::new(stream))?;
        } else if !mode.debug_reader && mode.debug_writer {
            self.run_game(BufReader::new(stream), io::stdout())?;
        } else if mode.debug_reader && mode.debug_writer {
            self.run_game(io::stdin(), io::stdout())?;
        } else {
            let reader = BufReader::new(stream.try_clone()?);
            let writer = BufWriter::new(stream);
            self.run_game(reader, writer)?;
        }
        
        Ok(())
    }
    
    /// Blocks the thread and parses/handles game messages
    /// from the provided reader.
    fn run_game(mut self, reader: impl Read, writer: impl Write) -> SCResult<()> {
        // TODO: Implement client loop
        Ok(())
    }
}