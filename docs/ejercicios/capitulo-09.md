# Ejercicio: salida lateral de un pipeline completo

Usa un `SimulatedDecoder` con dos frames y una detección sintética que supera
un `DetectionThreshold`. Para cada frame, crea un `FrameOutput` y agrega una
`Annotation` lateral. Comprueba que la secuencia del frame se conserva y que
la anotación mantiene etiqueta, caja y confianza.

Después plantea el caso de una detección bajo el umbral: la salida debe seguir
existiendo, pero sin anotaciones. Explica por qué este diseño permite que un
renderizador futuro sea una dependencia separada y por qué el ejercicio no
guarda ningún resultado.

La [solución](../../examples/soluciones/capitulo-09.rs) demuestra ese recorrido
determinista con una detección aceptada y otra rechazada.
