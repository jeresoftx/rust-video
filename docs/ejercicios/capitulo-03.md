# Ejercicio: falla local o cancelación

## Contexto

Una etapa recibe los frames `1`, `2` y `3`. El frame `2` no puede procesarse,
pero esa falla no invalida el resto de la secuencia. En un segundo escenario,
el frame `2` representa una cancelación solicitada por quien opera el sistema.

## Reto

Implementa una etapa que:

1. devuelva `StageOutcome::Failed` para el frame `2` y permita que el frame
   `3` avance;
2. cambie a `StageOutcome::Cancelled` para el frame `2` y demuestre que el
   frame `3` ya no se procesa.

## Preguntas de criterio

1. ¿Por qué una falla local debe conservarse en el reporte?
2. ¿Por qué cancelar no es equivalente a descartar un frame?
3. ¿Qué responsabilidad tendría una cola acotada que este ejemplo no modela?

La [solución](../../examples/soluciones/capitulo-03.rs) conserva ambos casos
en un pipeline puramente secuencial.
