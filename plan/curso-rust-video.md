# Plan De Curso: Rust Video

**Estado:** aprobado para planificación; implementación pendiente.

**Fuente de alcance:** RFC-0001 §10, "Procesamiento de video en streaming".

## Propósito

Construir un curso aplicado de nueve capítulos sobre un pipeline de video en
streaming con detección en tiempo real. Integra conocimientos canónicos de
performance, low-level, concurrencia, redes y AI engineering sin duplicarlos.

## Decisión De Alcance

**Concepto:** un sistema de video en tiempo real es un pipeline de frames,
tiempo, memoria, inferencia y salida; no solo una llamada a FFmpeg o un modelo.

**Problema:** enfocarse en una API oculta latencia acumulada, buffers, pérdida
de frames, costo de copia, modelos imperfectos y límites de privacidad.

**Alternativas:** construir un reproductor completo, integrar proveedores
reales desde el inicio o modelar un pipeline determinista antes de las
dependencias externas.

**Justificación:** el primer draft usará contratos y ejemplos reproducibles;
FFmpeg y la inferencia se introducirán solo después de justificar dependencias,
datos y presupuesto de rendimiento de cada capítulo.

## Reglas De Implementación

- Un issue equivale a una rama, un commit principal y un PR.
- Cada capítulo contiene explicación, diagrama Mermaid, ejemplos, ejercicios,
  soluciones, pruebas y benchmark o decisión documentada de no aplicar.
- No se usa `unsafe`, nightly ni dependencias externas no triviales sin
  autorización humana.
- No se procesan streams privados, rostros ni audio real sin autorización y
  documentación de privacidad.
- Ningún capítulo se marca como `reviewed` o `published` sin revisión humana.

## Capítulos Y Dependencias

### 01. Fundamentos de video

- [x] Explicar frames, resolución, FPS, códecs y contenedores.
- [x] Comparar datos crudos, códec y contenedor con sus tradeoffs.
- [x] Modelar metadatos de frame sin decodificar video real.

### 02. Captura e ingesta de streams

- [ ] Modelar fuentes, timestamps, buffers y pérdida de frames.
- [x] Referenciar protocolos canónicos desde `rust-networking`.
- [x] Documentar la diferencia entre ingesta y reproducción.

### 03. Pipeline de procesamiento en tiempo real

- [x] Modelar etapas, contratos y backpressure.
- [x] Comparar pipeline secuencial, por lotes y por etapas concurrentes.
- [x] Probar orden, cancelación y recuperación de fallas locales.

### 04. Decodificación y FFmpeg desde Rust

- [x] Explicar dónde inicia y termina la responsabilidad de FFmpeg.
- [x] Diseñar un adaptador que pueda simularse sin FFmpeg instalado.
- [x] Documentar la dependencia externa y sus riesgos de plataforma.

### 05. Detección de objetos sobre frames

- [x] Explicar inferencia, umbrales de confianza y falsos positivos.
- [x] Modelar resultados de detección sin incorporar un modelo aún.
- [x] Documentar privacidad, evaluación y límites de precisión.

### 06. Seguimiento entre frames

- [x] Modelar identidad temporal, asociación y pérdida de tracking.
- [ ] Comparar detección independiente y tracking con estado.
- [x] Comparar detección independiente y tracking con estado.
- [x] Probar oclusión, expiración y reasignación explícitamente.

### 07. Rendimiento y presupuesto de latencia

- [x] Expresar un presupuesto por etapa y latencia de extremo a extremo.
- [ ] Medir el modelo local y documentar qué no representa.
- [x] Comparar throughput, latencia y calidad de salida.

### 08. Procesamiento paralelo y zero-copy

- [ ] Modelar propiedad de buffers, paralelismo y backpressure.
- [ ] Conectar con canónicos de concurrencia y low-level.
- [ ] Documentar por qué zero-copy no justifica `unsafe` por defecto.

### 09. Salida y anotación del stream

- [ ] Modelar overlays, resultados de inferencia y salida de pipeline.
- [ ] Comparar anotación destructiva, metadatos laterales y privacidad.
- [ ] Cerrar con una demostración determinista de pipeline completo.

## Auditoría Y Revisión

- [ ] Auditar crate, enlaces canónicos, manifest, diagramas, ejemplos,
      ejercicios, soluciones, benchmarks y límites de privacidad.
- [ ] Solicitar revisión humana editorial, técnica y de privacidad.

## Alcance Futuro No Activo

La RFC-0004 está en borrador y propone sincronización A/V, protocolos de
streaming, calidad adaptativa, aceleración, observabilidad, grabación,
privacidad, fondos virtuales, gestos y subtítulos. No se crean issues ni se
implementa ese alcance mientras la RFC no se active.
