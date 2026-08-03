# Ejercicio: decidir qué frame se conserva

## Contexto

Una fuente simulada produce los frames `10`, `11` y `12`. El consumidor se
retrasa y el buffer solo puede conservar dos frames. Para un escenario de
tiempo real, la política aprobada descarta el más antiguo.

## Reto

Construye un `FrameBuffer` de capacidad `2`, inserta los tres frames e
imprime el frame descartado y los que el consumidor procesa. El resultado debe
conservar `11` y `12`.

## Preguntas de criterio

1. ¿Por qué el descarte debe ser un valor de retorno y no un efecto silencioso?
2. ¿Qué cambiaría si el caso fuera análisis histórico en lugar de tiempo real?
3. ¿Qué información adicional necesitaría una política basada en tiempo y no
   solo en cantidad de frames?

La [solución](../../examples/soluciones/capitulo-02.rs) utiliza la política
actual sin abrir una cámara ni una conexión de red.
