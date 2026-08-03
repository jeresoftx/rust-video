//! Solución del ejercicio del capítulo 04.

use rust_video::{DecodeError, FrameDecoder, SimulatedDecoder};

fn main() {
    let mut decoder = SimulatedDecoder::with_failure([], DecodeError::InputUnavailable);

    match decoder.decode_next() {
        Ok(Some(frame)) => println!("frame={}", frame.sequence()),
        Ok(None) => println!("fin de entrada"),
        Err(DecodeError::InputUnavailable) => println!("entrada no disponible"),
        Err(DecodeError::InvalidInput) => println!("entrada inválida"),
    }
}
