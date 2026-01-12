use rodio::{self, OutputStream, OutputStreamBuilder, Sink};
use core::error;
use std::fs::File;


pub struct Audio{
    pub sink : Sink,
    _stream : OutputStream,
}

impl Audio{
    pub fn build() -> Self{
        let output_stream = OutputStreamBuilder::open_default_stream().expect("erro ao iniciar o output_stream");
        Audio {
            sink : rodio::Sink::connect_new(output_stream.mixer()),
            _stream : output_stream,
        }
    }

    pub fn start_msc(&mut self, file : File) -> Result<(), Box<dyn error::Error>>{
        self.sink.append(rodio::Decoder::try_from(file)?);
        Ok(())
    }
}


