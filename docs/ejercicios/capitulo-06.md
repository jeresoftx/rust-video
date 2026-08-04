# Ejercicio: oclusión, expiración y reasignación

Un track tiene tolerancia de una ausencia. Simula dos frames sin asociación y
comprueba que el track expira. Después intenta observarlo de nuevo y crea un
nuevo `TrackId` para representar la reasignación explícita.

La [solución](../../examples/soluciones/capitulo-06.rs) demuestra que una
identidad expirada no se reutiliza de forma silenciosa.
