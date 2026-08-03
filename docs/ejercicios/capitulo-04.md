# Ejercicio: adaptar una dependencia externa

## Reto

Usa `SimulatedDecoder` para consumir dos frames y distinguir tres resultados:
un frame disponible, final de entrada y `DecodeError::InputUnavailable`.

## Preguntas de criterio

1. ¿Por qué el pipeline depende del trait `FrameDecoder` y no de FFmpeg?
2. ¿Qué pruebas podrían ejecutarse sin una instalación nativa?
3. ¿Qué decisión humana haría falta antes de agregar un wrapper de FFmpeg?

La [solución](../../examples/soluciones/capitulo-04.rs) conserva el contrato
del dominio y hace visible el error.
