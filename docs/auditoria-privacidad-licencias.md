# Auditoría de privacidad, licencias y límites de integración

## Propósito

Comprobar que el draft no convierta ejemplos pedagógicos en una integración de
medios reales ni presente derechos o garantías que el repositorio no tiene.

## Resultado de la autoauditoría

- Los ejemplos usan `FrameMetadata`, payloads de bytes definidos en el código
  y detecciones sintéticas; no abren cámara, archivo, socket ni stream privado.
- No hay rostros, embeddings, identificación de personas, audio, transcripción
  ni datos biométricos en código, ejemplos o pruebas.
- `Cargo.toml` no incorpora FFmpeg, bindings nativas, runtimes de inferencia,
  modelos, SDK de cámara ni cliente de red.
- Los capítulos 04, 05, 06 y 09 describen de forma explícita los límites de
  FFmpeg, inferencia, tracking y salida antes de una posible integración.
- `LICENSE.md` separa el código bajo MIT OR Apache-2.0 del contenido educativo
  bajo CC BY-SA 4.0, con los textos de licencia incluidos en el repositorio.

## Gatillos para una decisión humana futura

Antes de integrar una fuente de video, FFmpeg, un modelo o una salida remota se
debe documentar la justificación de dependencia, plataforma, licencia,
proveniencia de datos, política de privacidad, retención, seguridad y pruebas.
No basta con que el contrato actual compile.

## Límite de esta auditoría

Esta evidencia corresponde al draft sintético actual. No certifica cumplimiento
legal, idoneidad para producción ni autorización para tratar medios de terceros.
La revisión humana de privacidad sigue pendiente.
