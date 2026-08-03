//! Procesamiento de video en streaming para Jeresoft Academy.
//!
//! El crate acompañará el curso `rust-video`. Sus implementaciones partirán de
//! contratos reproducibles de pipeline y orquestación; no reimplementarán
//! códecs, protocolos ni modelos de visión.

#![forbid(unsafe_code)]

use std::{collections::VecDeque, time::Duration};

/// Dimensiones de un frame expresadas en píxeles.
///
/// La resolución solo describe la geometría. No implica un formato de color,
/// un tamaño de buffer ni una calidad visual determinada.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VideoResolution {
    width: u32,
    height: u32,
}

impl VideoResolution {
    /// Crea una resolución cuando ambas dimensiones son mayores que cero.
    pub const fn new(width: u32, height: u32) -> Option<Self> {
        if width == 0 || height == 0 {
            return None;
        }

        Some(Self { width, height })
    }

    /// Devuelve el ancho en píxeles.
    pub const fn width(self) -> u32 {
        self.width
    }

    /// Devuelve el alto en píxeles.
    pub const fn height(self) -> u32 {
        self.height
    }
}

/// Información temporal y geométrica de un frame sin conservar sus píxeles.
///
/// El modelo permite probar orden, pérdida y latencia de forma determinista
/// antes de conectar una fuente de video o un decodificador real.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameMetadata {
    sequence: u64,
    timestamp: Duration,
    resolution: VideoResolution,
}

impl FrameMetadata {
    /// Crea los metadatos de un frame emitido en un instante dado.
    pub const fn new(sequence: u64, timestamp: Duration, resolution: VideoResolution) -> Self {
        Self {
            sequence,
            timestamp,
            resolution,
        }
    }

    /// Devuelve el número de secuencia asignado por la fuente.
    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    /// Devuelve la marca de tiempo relativa del frame.
    pub const fn timestamp(self) -> Duration {
        self.timestamp
    }

    /// Devuelve la resolución declarada para el frame.
    pub const fn resolution(self) -> VideoResolution {
        self.resolution
    }
}

/// Fuente que puede entregar el siguiente frame disponible.
///
/// El trait representa el límite entre una integración de captura o transporte
/// y el pipeline. El curso no proporciona una fuente de cámara o red real.
pub trait FrameSource {
    /// Devuelve el siguiente frame cuando la fuente tiene uno disponible.
    fn next_frame(&mut self) -> Option<FrameMetadata>;
}

/// Buffer acotado que conserva los frames más recientes.
///
/// Cuando la capacidad se agota, insertar un frame nuevo descarta y devuelve
/// el frame más antiguo. De ese modo la pérdida es una decisión observable.
#[derive(Debug)]
pub struct FrameBuffer {
    capacity: usize,
    frames: VecDeque<FrameMetadata>,
}

impl FrameBuffer {
    /// Crea un buffer con una capacidad positiva.
    pub fn new(capacity: usize) -> Option<Self> {
        if capacity == 0 {
            return None;
        }

        Some(Self {
            capacity,
            frames: VecDeque::with_capacity(capacity),
        })
    }

    /// Inserta un frame y devuelve el que se descartó, si la capacidad se agotó.
    pub fn push(&mut self, frame: FrameMetadata) -> Option<FrameMetadata> {
        let discarded = if self.frames.len() == self.capacity {
            self.frames.pop_front()
        } else {
            None
        };
        self.frames.push_back(frame);
        discarded
    }

    /// Extrae el frame más antiguo que sigue disponible.
    pub fn pop_front(&mut self) -> Option<FrameMetadata> {
        self.frames.pop_front()
    }

    /// Indica cuántos frames conserva el buffer.
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Indica si no hay frames pendientes.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

/// Declara que el crate base se puede enlazar antes de introducir capítulos.
pub fn course_status() -> &'static str {
    "planned"
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{course_status, FrameBuffer, FrameMetadata, VideoResolution};

    #[test]
    fn crate_base_declares_el_estado_planeado() {
        assert_eq!(course_status(), "planned");
    }

    #[test]
    fn rechaza_resoluciones_con_dimension_cero() {
        assert!(VideoResolution::new(0, 1_080).is_none());
        assert!(VideoResolution::new(1_920, 0).is_none());
    }

    #[test]
    fn conserva_los_metadatos_de_un_frame() {
        let resolution = VideoResolution::new(1_920, 1_080).expect("resolución válida");
        let timestamp = Duration::from_millis(42);
        let frame = FrameMetadata::new(7, timestamp, resolution);

        assert_eq!(frame.sequence(), 7);
        assert_eq!(frame.timestamp(), timestamp);
        assert_eq!(frame.resolution(), resolution);
    }

    #[test]
    fn rechaza_buffers_sin_capacidad() {
        assert!(FrameBuffer::new(0).is_none());
    }

    #[test]
    fn descarta_el_frame_mas_antiguo_cuando_el_buffer_se_llena() {
        let resolution = VideoResolution::new(640, 480).expect("resolución válida");
        let first = FrameMetadata::new(1, Duration::from_millis(0), resolution);
        let second = FrameMetadata::new(2, Duration::from_millis(33), resolution);
        let third = FrameMetadata::new(3, Duration::from_millis(66), resolution);
        let mut buffer = FrameBuffer::new(2).expect("capacidad válida");

        assert_eq!(buffer.push(first), None);
        assert_eq!(buffer.push(second), None);
        assert_eq!(buffer.push(third), Some(first));
        assert_eq!(buffer.pop_front(), Some(second));
        assert_eq!(buffer.pop_front(), Some(third));
        assert!(buffer.is_empty());
    }
}
