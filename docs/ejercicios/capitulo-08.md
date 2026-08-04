# Ejercicio: propiedad compartida y trabajo en vuelo

Declara un `ProcessingPlan` con dos workers y una capacidad de tres frames en
vuelo. Comprueba qué ocurre al intentar aceptar el tercer y el cuarto frame.
Después crea un `SharedFramePayload`, clónalo para dos etapas y confirma que
ambas referencias comparten almacenamiento.

Explica con tus palabras dónde comienza el ahorro de copias y por qué la
conversión inicial desde una fuente aún puede asignar o copiar. No uses
`unsafe`: el objetivo es razonar sobre propiedad y límites antes de perseguir
una optimización de bajo nivel.

La [solución](../../examples/soluciones/capitulo-08.rs) conserva el contrato
seguro y separa la política de backpressure de la vida útil de los bytes.
