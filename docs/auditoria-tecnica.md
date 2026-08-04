# Auditoría técnica del draft

## Propósito

Verificar que los contratos pedagógicos se mantengan compilables, seguros y
honestos sobre lo que todavía no integran.

## Resultado de la autoauditoría

- `Cargo.toml` declara solo el crate local: no hay dependencias directas ni
  transitivas en `Cargo.lock`.
- El crate prohíbe `unsafe` con `#![forbid(unsafe_code)]`.
- Los contratos cubren metadatos, buffer acotado, pipeline, decodificación
  simulada, detección, tracking, latencia, payload compartido y salida lateral.
- Los ejemplos y soluciones compilan con la biblioteca estándar; no usan
  FFmpeg, cámara, red, runtime asíncrono ni modelo de IA real.
- Las decisiones de benchmark y property testing están escritas por capítulo;
  no se agregó una dependencia de benchmarking sin una pregunta representativa.
- Las verificaciones aplicables se ejecutaron en verde: formato, Clippy
  estricto, pruebas de todos los targets, doctests y comprobación de diff.

## Límite de esta auditoría

El resultado verifica el draft local, no rendimiento de producción ni
compatibilidad con FFmpeg, plataformas, cámaras, GPU o proveedores externos.
La revisión humana técnica sigue pendiente.
