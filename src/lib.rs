//! Procesamiento de video en streaming para Jeresoft Academy.
//!
//! El crate acompañará el curso `rust-video`. Sus implementaciones partirán de
//! contratos reproducibles de pipeline y orquestación; no reimplementarán
//! códecs, protocolos ni modelos de visión.

#![forbid(unsafe_code)]

use std::{collections::VecDeque, sync::Arc, time::Duration};

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

/// Bytes de un frame que pueden compartirse entre etapas de solo lectura.
///
/// La conversión inicial delimita la propiedad de la fuente. Las clonaciones
/// posteriores comparten la misma asignación mediante `Arc` y no duplican los
/// bytes; ello no elimina por sí solo todas las asignaciones del sistema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedFramePayload {
    bytes: Arc<[u8]>,
}

impl SharedFramePayload {
    /// Convierte los bytes de entrada en un payload compartible.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Arc::from(bytes),
        }
    }

    /// Devuelve los bytes como una vista de solo lectura.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Devuelve la cantidad de bytes disponibles en el payload.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Indica si el payload no contiene bytes.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Indica si dos payloads comparten la misma asignación de bytes.
    pub fn shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bytes, &other.bytes)
    }
}

/// Límites explícitos para trabajo concurrente dentro del modelo local.
///
/// El plan no crea hilos ni garantiza rendimiento. Expresa el número máximo de
/// workers y de frames que una integración puede dejar en vuelo antes de aplicar
/// la política de backpressure que corresponda.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessingPlan {
    worker_count: usize,
    maximum_in_flight: usize,
}

impl ProcessingPlan {
    /// Crea un plan cuando workers y capacidad en vuelo son positivos.
    pub const fn new(worker_count: usize, maximum_in_flight: usize) -> Option<Self> {
        if worker_count == 0 || maximum_in_flight == 0 {
            return None;
        }

        Some(Self {
            worker_count,
            maximum_in_flight,
        })
    }

    /// Devuelve cuántos workers declara el plan.
    pub const fn worker_count(self) -> usize {
        self.worker_count
    }

    /// Devuelve el máximo de frames permitidos en vuelo.
    pub const fn maximum_in_flight(self) -> usize {
        self.maximum_in_flight
    }

    /// Indica si una cola con el trabajo dado puede aceptar otro frame.
    pub const fn can_accept(self, in_flight: usize) -> bool {
        in_flight < self.maximum_in_flight
    }
}

/// Falla local de una etapa que puede registrarse sin cancelar el pipeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StageFailure {
    message: &'static str,
}

impl StageFailure {
    /// Crea una falla con un mensaje estático de diagnóstico.
    pub const fn new(message: &'static str) -> Self {
        Self { message }
    }

    /// Devuelve el mensaje de diagnóstico.
    pub const fn message(self) -> &'static str {
        self.message
    }
}

/// Resultado explícito de procesar un frame en una etapa.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageOutcome {
    /// El frame puede avanzar a la siguiente etapa.
    Forward(FrameMetadata),
    /// La etapa descarta el frame según una decisión documentada.
    Dropped,
    /// El pipeline no debe procesar más frames.
    Cancelled,
    /// La etapa tuvo una falla local y el pipeline puede continuar.
    Failed(StageFailure),
}

/// Etapa síncrona que procesa un frame dentro de un pipeline.
pub trait FrameStage {
    /// Procesa un frame y declara cómo debe continuar el pipeline.
    fn process(&mut self, frame: FrameMetadata) -> StageOutcome;
}

/// Ejecuta una etapa sobre una secuencia finita y determinista de frames.
#[derive(Debug)]
pub struct Pipeline<S> {
    stage: S,
}

impl<S> Pipeline<S>
where
    S: FrameStage,
{
    /// Crea un pipeline secuencial con una etapa.
    pub const fn new(stage: S) -> Self {
        Self { stage }
    }

    /// Ejecuta la etapa y conserva evidencia de los resultados observables.
    pub fn run<I>(mut self, frames: I) -> PipelineReport
    where
        I: IntoIterator<Item = FrameMetadata>,
    {
        let mut report = PipelineReport::default();

        for frame in frames {
            match self.stage.process(frame) {
                StageOutcome::Forward(frame) => report.forwarded_sequences.push(frame.sequence()),
                StageOutcome::Dropped => report.dropped_frames += 1,
                StageOutcome::Failed(failure) => report.failures.push(failure),
                StageOutcome::Cancelled => {
                    report.cancelled = true;
                    break;
                }
            }
        }

        report
    }
}

/// Evidencia de una ejecución secuencial del pipeline.
#[derive(Debug, Default)]
pub struct PipelineReport {
    forwarded_sequences: Vec<u64>,
    dropped_frames: usize,
    failures: Vec<StageFailure>,
    cancelled: bool,
}

impl PipelineReport {
    /// Devuelve las secuencias que avanzaron en el mismo orden de entrada.
    pub fn forwarded_sequences(&self) -> &[u64] {
        &self.forwarded_sequences
    }

    /// Devuelve cuántos frames se descartaron por una etapa.
    pub const fn dropped_frames(&self) -> usize {
        self.dropped_frames
    }

    /// Devuelve las fallas locales registradas durante la ejecución.
    pub fn failures(&self) -> &[StageFailure] {
        &self.failures
    }

    /// Indica si una etapa canceló el recorrido.
    pub const fn was_cancelled(&self) -> bool {
        self.cancelled
    }
}

/// Errores que una integración de decodificación puede comunicar al dominio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    /// La entrada o la dependencia externa no está disponible.
    InputUnavailable,
    /// La entrada no pudo interpretarse bajo el contrato del adaptador.
    InvalidInput,
}

/// Adaptador que entrega frames decodificados sin revelar la implementación externa.
pub trait FrameDecoder {
    /// Decodifica el siguiente frame, indica final de entrada o comunica un error.
    fn decode_next(&mut self) -> Result<Option<FrameMetadata>, DecodeError>;
}

/// Decodificador determinista para ejemplos y pruebas del curso.
#[derive(Debug)]
pub struct SimulatedDecoder {
    frames: VecDeque<FrameMetadata>,
    pending_failure: Option<DecodeError>,
}

impl SimulatedDecoder {
    /// Crea un decodificador que entrega los frames proporcionados en su orden original.
    pub fn new<I>(frames: I) -> Self
    where
        I: IntoIterator<Item = FrameMetadata>,
    {
        Self {
            frames: frames.into_iter().collect(),
            pending_failure: None,
        }
    }

    /// Crea un decodificador que comunica una falla antes de leer su entrada.
    pub fn with_failure<I>(frames: I, failure: DecodeError) -> Self
    where
        I: IntoIterator<Item = FrameMetadata>,
    {
        Self {
            frames: frames.into_iter().collect(),
            pending_failure: Some(failure),
        }
    }
}

impl FrameDecoder for SimulatedDecoder {
    fn decode_next(&mut self) -> Result<Option<FrameMetadata>, DecodeError> {
        if let Some(failure) = self.pending_failure.take() {
            return Err(failure);
        }

        Ok(self.frames.pop_front())
    }
}

/// Región rectangular de una detección sintética expresada en píxeles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundingBox {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl BoundingBox {
    /// Crea una región con dimensiones positivas.
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Option<Self> {
        if width == 0 || height == 0 {
            return None;
        }

        Some(Self {
            x,
            y,
            width,
            height,
        })
    }

    /// Devuelve la coordenada horizontal del origen.
    pub const fn x(self) -> u32 {
        self.x
    }

    /// Devuelve la coordenada vertical del origen.
    pub const fn y(self) -> u32 {
        self.y
    }

    /// Devuelve el ancho de la región.
    pub const fn width(self) -> u32 {
        self.width
    }

    /// Devuelve el alto de la región.
    pub const fn height(self) -> u32 {
        self.height
    }
}

/// Resultado sintético de una etapa de detección.
#[derive(Clone, Debug, PartialEq)]
pub struct Detection {
    label: String,
    area: BoundingBox,
    confidence: f32,
}

impl Detection {
    /// Crea una detección cuando etiqueta y confianza respetan el contrato.
    pub fn new(label: impl Into<String>, area: BoundingBox, confidence: f32) -> Option<Self> {
        let label = label.into();
        if label.is_empty() || !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return None;
        }

        Some(Self {
            label,
            area,
            confidence,
        })
    }

    /// Devuelve la etiqueta declarada por el adaptador de inferencia.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Devuelve la región declarada para la detección.
    pub const fn area(&self) -> BoundingBox {
        self.area
    }

    /// Devuelve la confianza normalizada de la detección.
    pub const fn confidence(&self) -> f32 {
        self.confidence
    }
}

/// Umbral explícito para aceptar detecciones sintéticas.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DetectionThreshold(f32);

impl DetectionThreshold {
    /// Crea un umbral finito entre cero y uno, inclusive.
    pub fn new(value: f32) -> Option<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return None;
        }

        Some(Self(value))
    }

    /// Indica si la confianza de una detección alcanza este umbral.
    pub fn accepts(self, detection: &Detection) -> bool {
        detection.confidence() >= self.0
    }
}

/// Anotación lateral derivada de una detección ya aceptada.
///
/// La anotación no modifica bytes de imagen. Conserva solo los datos mínimos
/// que un consumidor podría renderizar o inspeccionar bajo su propio contrato.
#[derive(Clone, Debug, PartialEq)]
pub struct Annotation {
    label: String,
    area: BoundingBox,
    confidence: f32,
}

impl Annotation {
    /// Crea una anotación lateral a partir de una detección existente.
    pub fn from_detection(detection: &Detection) -> Self {
        Self {
            label: detection.label().to_owned(),
            area: detection.area(),
            confidence: detection.confidence(),
        }
    }

    /// Devuelve la etiqueta que se asociará visualmente al frame.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Devuelve la región de la anotación.
    pub const fn area(&self) -> BoundingBox {
        self.area
    }

    /// Devuelve la confianza declarada por la detección de origen.
    pub const fn confidence(&self) -> f32 {
        self.confidence
    }
}

/// Salida determinista de un frame y sus anotaciones laterales.
///
/// El tipo no contiene píxeles, no renderiza overlays y no persiste datos. Su
/// única responsabilidad es mantener la relación verificable entre frame y
/// resultados de análisis.
#[derive(Debug)]
pub struct FrameOutput {
    frame: FrameMetadata,
    annotations: Vec<Annotation>,
}

impl FrameOutput {
    /// Crea una salida vacía para un frame ya identificado.
    pub const fn new(frame: FrameMetadata) -> Self {
        Self {
            frame,
            annotations: Vec::new(),
        }
    }

    /// Añade una anotación lateral sin modificar el frame de origen.
    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
    }

    /// Devuelve los metadatos del frame asociado a la salida.
    pub const fn frame(&self) -> FrameMetadata {
        self.frame
    }

    /// Devuelve las anotaciones en el orden en que se agregaron.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// Identificador asignado por una decisión de asociación explícita.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrackId(u64);

impl TrackId {
    /// Crea un identificador de track.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor estable del identificador.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Estado observable de una identidad temporal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrackStatus {
    /// El track recibió una observación reciente.
    Active,
    /// El track sigue reservado durante una oclusión limitada.
    Occluded,
    /// El track ya no acepta asociaciones y requiere reasignación explícita.
    Expired,
}

/// Estado mínimo de un track entre frames.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Track {
    id: TrackId,
    last_frame: FrameMetadata,
    missed_frames: u32,
    max_missed_frames: u32,
    status: TrackStatus,
}

impl Track {
    /// Crea un track activo con una tolerancia de oclusión finita.
    pub const fn new(id: TrackId, first_frame: FrameMetadata, max_missed_frames: u32) -> Self {
        Self {
            id,
            last_frame: first_frame,
            missed_frames: 0,
            max_missed_frames,
            status: TrackStatus::Active,
        }
    }

    /// Devuelve el identificador de esta identidad temporal.
    pub const fn id(self) -> TrackId {
        self.id
    }

    /// Devuelve el estado actual del track.
    pub const fn status(self) -> TrackStatus {
        self.status
    }

    /// Registra una observación asociada y devuelve si el track pudo aceptarla.
    pub fn observe(&mut self, frame: FrameMetadata) -> bool {
        if self.status == TrackStatus::Expired {
            return false;
        }

        self.last_frame = frame;
        self.missed_frames = 0;
        self.status = TrackStatus::Active;
        true
    }

    /// Registra la ausencia de una asociación y actualiza el estado de oclusión.
    pub fn mark_missing(&mut self) -> TrackStatus {
        if self.status == TrackStatus::Expired {
            return self.status;
        }

        self.missed_frames = self.missed_frames.saturating_add(1);
        self.status = if self.missed_frames > self.max_missed_frames {
            TrackStatus::Expired
        } else {
            TrackStatus::Occluded
        };
        self.status
    }
}

/// Duración observada o estimada para una etapa del recorrido local.
///
/// No representa una garantía de producción: su propósito es hacer explícito
/// qué partes componen el presupuesto de latencia que se está razonando.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StageLatency {
    name: &'static str,
    duration: Duration,
}

impl StageLatency {
    /// Crea una duración para una etapa identificada por un nombre no vacío.
    pub const fn new(name: &'static str, duration: Duration) -> Option<Self> {
        if name.is_empty() {
            return None;
        }

        Some(Self { name, duration })
    }

    /// Devuelve el nombre de la etapa presupuestada.
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Devuelve la duración registrada para la etapa.
    pub const fn duration(self) -> Duration {
        self.duration
    }
}

/// Presupuesto local de latencia compuesto por etapas explícitas.
///
/// El presupuesto no mide una cámara, FFmpeg, una GPU ni una red. Solo permite
/// sumar y comprobar duraciones que el ejemplo o quien estudia declaró.
#[derive(Debug)]
pub struct LatencyBudget {
    limit: Duration,
    stages: Vec<StageLatency>,
}

impl LatencyBudget {
    /// Crea un presupuesto con un límite positivo de latencia extremo a extremo.
    pub fn new(limit: Duration) -> Option<Self> {
        if limit.is_zero() {
            return None;
        }

        Some(Self {
            limit,
            stages: Vec::new(),
        })
    }

    /// Añade una etapa al presupuesto cuando su nombre es válido.
    pub fn add_stage(&mut self, name: &'static str, duration: Duration) -> bool {
        let Some(stage) = StageLatency::new(name, duration) else {
            return false;
        };

        self.stages.push(stage);
        true
    }

    /// Devuelve el límite total que se decidió para este recorrido.
    pub const fn limit(&self) -> Duration {
        self.limit
    }

    /// Devuelve las etapas que componen el presupuesto en orden de registro.
    pub fn stages(&self) -> &[StageLatency] {
        &self.stages
    }

    /// Suma las duraciones declaradas por etapa sin producir una medición externa.
    pub fn total(&self) -> Duration {
        self.stages.iter().fold(Duration::ZERO, |total, stage| {
            total.saturating_add(stage.duration())
        })
    }

    /// Indica si las etapas declaradas caben dentro del límite elegido.
    pub fn is_within_limit(&self) -> bool {
        self.total() <= self.limit
    }
}

/// Declara que el crate contiene un draft completo pendiente de revisión humana.
pub fn course_status() -> &'static str {
    "draft"
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{
        course_status, Annotation, BoundingBox, DecodeError, Detection, DetectionThreshold,
        FrameBuffer, FrameDecoder, FrameMetadata, FrameOutput, FrameStage, LatencyBudget, Pipeline,
        ProcessingPlan, SharedFramePayload, SimulatedDecoder, StageFailure, StageLatency,
        StageOutcome, Track, TrackId, TrackStatus, VideoResolution,
    };

    #[test]
    fn crate_declara_el_estado_de_borrador() {
        assert_eq!(course_status(), "draft");
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

    #[test]
    fn comparte_el_payload_sin_duplicar_su_almacenamiento_despues_de_la_entrada() {
        let payload = SharedFramePayload::from_bytes(vec![1, 2, 3, 4]);
        let stage_copy = payload.clone();

        assert_eq!(stage_copy.as_slice(), &[1, 2, 3, 4]);
        assert!(payload.shares_storage_with(&stage_copy));
        assert!(!payload.is_empty());
    }

    #[test]
    fn hace_visible_el_limite_de_trabajo_en_vuelo() {
        assert!(ProcessingPlan::new(0, 1).is_none());
        assert!(ProcessingPlan::new(1, 0).is_none());

        let plan = ProcessingPlan::new(2, 3).expect("plan válido");
        assert_eq!(plan.worker_count(), 2);
        assert_eq!(plan.maximum_in_flight(), 3);
        assert!(plan.can_accept(2));
        assert!(!plan.can_accept(3));
    }

    #[derive(Default)]
    struct FallaEnLaSecuencia {
        failure_sequence: u64,
        cancel_sequence: Option<u64>,
    }

    impl FrameStage for FallaEnLaSecuencia {
        fn process(&mut self, frame: FrameMetadata) -> StageOutcome {
            if self.cancel_sequence == Some(frame.sequence()) {
                return StageOutcome::Cancelled;
            }
            if self.failure_sequence == frame.sequence() {
                return StageOutcome::Failed(StageFailure::new("falla simulada"));
            }

            StageOutcome::Forward(frame)
        }
    }

    #[test]
    fn conserva_orden_y_recupera_una_falla_local() {
        let resolution = VideoResolution::new(640, 480).expect("resolución válida");
        let frames = (1..=3).map(|sequence| {
            FrameMetadata::new(sequence, Duration::from_millis(sequence * 33), resolution)
        });
        let stage = FallaEnLaSecuencia {
            failure_sequence: 2,
            cancel_sequence: None,
        };

        let report = Pipeline::new(stage).run(frames);

        assert_eq!(report.forwarded_sequences(), &[1, 3]);
        assert_eq!(report.failures().len(), 1);
        assert!(!report.was_cancelled());
    }

    #[test]
    fn detiene_el_recorrido_cuando_una_etapa_cancela() {
        let resolution = VideoResolution::new(640, 480).expect("resolución válida");
        let frames = (1..=3).map(|sequence| {
            FrameMetadata::new(sequence, Duration::from_millis(sequence * 33), resolution)
        });
        let stage = FallaEnLaSecuencia {
            failure_sequence: 0,
            cancel_sequence: Some(2),
        };

        let report = Pipeline::new(stage).run(frames);

        assert_eq!(report.forwarded_sequences(), &[1]);
        assert!(report.was_cancelled());
    }

    #[test]
    fn el_decodificador_simulado_entrega_frames_en_orden() {
        let resolution = VideoResolution::new(640, 480).expect("resolución válida");
        let frames = [
            FrameMetadata::new(1, Duration::ZERO, resolution),
            FrameMetadata::new(2, Duration::from_millis(33), resolution),
        ];
        let mut decoder = SimulatedDecoder::new(frames);

        assert_eq!(
            decoder
                .decode_next()
                .expect("sin error")
                .map(FrameMetadata::sequence),
            Some(1)
        );
        assert_eq!(
            decoder
                .decode_next()
                .expect("sin error")
                .map(FrameMetadata::sequence),
            Some(2)
        );
        assert_eq!(decoder.decode_next().expect("sin error"), None);
    }

    #[test]
    fn el_decodificador_expone_una_falla_sin_ocultar_su_tipo() {
        let mut decoder = SimulatedDecoder::with_failure([], DecodeError::InputUnavailable);

        assert_eq!(decoder.decode_next(), Err(DecodeError::InputUnavailable));
    }

    #[test]
    fn rechaza_cajas_y_confianzas_fuera_del_contrato() {
        assert!(BoundingBox::new(0, 0, 0, 20).is_none());
        assert!(DetectionThreshold::new(-0.1).is_none());
        assert!(DetectionThreshold::new(1.1).is_none());
    }

    #[test]
    fn aplica_un_umbral_explicito_a_una_deteccion_sintetica() {
        let area = BoundingBox::new(10, 20, 30, 40).expect("caja válida");
        let detection = Detection::new("objeto", area, 0.82).expect("confianza válida");
        let threshold = DetectionThreshold::new(0.8).expect("umbral válido");

        assert!(threshold.accepts(&detection));
        assert_eq!(detection.label(), "objeto");
        assert_eq!(detection.area(), area);
    }

    #[test]
    fn conserva_las_anotaciones_como_metadatos_laterales_del_frame() {
        let resolution = VideoResolution::new(640, 480).expect("resolución válida");
        let frame = FrameMetadata::new(12, Duration::from_millis(396), resolution);
        let area = BoundingBox::new(10, 20, 30, 40).expect("caja válida");
        let detection = Detection::new("objeto", area, 0.82).expect("detección válida");
        let mut output = FrameOutput::new(frame);

        output.add_annotation(Annotation::from_detection(&detection));

        assert_eq!(output.frame(), frame);
        assert_eq!(output.annotations().len(), 1);
        assert_eq!(output.annotations()[0].label(), "objeto");
        assert_eq!(output.annotations()[0].area(), area);
        assert_eq!(output.annotations()[0].confidence(), 0.82);
    }

    #[test]
    fn expira_un_track_tras_oclusiones_y_exige_reasignacion() {
        let resolution = VideoResolution::new(640, 480).expect("resolución válida");
        let first = FrameMetadata::new(1, Duration::ZERO, resolution);
        let second = FrameMetadata::new(2, Duration::from_millis(33), resolution);
        let mut track = Track::new(TrackId::new(7), first, 1);

        assert_eq!(track.mark_missing(), TrackStatus::Occluded);
        assert_eq!(track.mark_missing(), TrackStatus::Expired);
        assert!(!track.observe(second));
        assert_eq!(track.status(), TrackStatus::Expired);

        let reassigned = Track::new(TrackId::new(8), second, 1);
        assert_eq!(reassigned.id(), TrackId::new(8));
    }

    #[test]
    fn suma_etapas_y_hace_visible_el_tradeoff_de_latencia() {
        let mut budget = LatencyBudget::new(Duration::from_millis(45)).expect("límite válido");

        assert!(budget.add_stage("ingesta", Duration::from_millis(5)));
        assert!(budget.add_stage("análisis", Duration::from_millis(20)));
        assert!(budget.add_stage("salida", Duration::from_millis(8)));

        assert_eq!(budget.total(), Duration::from_millis(33));
        assert!(budget.is_within_limit());
        assert_eq!(budget.stages()[1].name(), "análisis");
        assert_eq!(budget.stages()[1].duration(), Duration::from_millis(20));
    }

    #[test]
    fn rechaza_un_limite_nulo_y_nombres_de_etapa_vacios() {
        assert!(LatencyBudget::new(Duration::ZERO).is_none());
        assert!(StageLatency::new("", Duration::from_millis(1)).is_none());

        let mut budget = LatencyBudget::new(Duration::from_millis(1)).expect("límite válido");
        assert!(!budget.add_stage("", Duration::ZERO));
        assert!(budget.add_stage("ingesta", Duration::from_millis(2)));
        assert!(!budget.is_within_limit());
    }
}
