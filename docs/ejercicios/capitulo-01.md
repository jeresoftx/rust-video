# Ejercicio: describir un frame sin píxeles

## Contexto

Una fuente simulada emitió el frame de secuencia `24` a los `800 ms`, con
resolución de `1280x720`. Aún no conocemos sus píxeles y no debemos inventar
un buffer para representarlos.

## Reto

Escribe un programa que construya un `FrameMetadata` para ese frame e imprima:

```text
frame=24 tiempo=800ms resolución=1280x720
```

## Preguntas de criterio

1. ¿Por qué la marca de tiempo es un `Duration` y no un FPS?
2. ¿Por qué una resolución `0x720` debe rechazarse antes de crear el frame?
3. ¿Qué información faltaría para reservar un buffer de píxeles de forma
   correcta?

La [solución](../../examples/soluciones/capitulo-01.rs) muestra una respuesta
posible. Léela después de intentar resolver el reto.
