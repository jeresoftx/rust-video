# Ejercicio: presupuesto y medición local

Parte de un límite extremo a extremo de 45 ms. Declara cuatro etapas con
duraciones que sumen menos del límite y confirma que el presupuesto cabe. Luego
agrega una etapa que lo exceda y explica qué decisión podrías tomar: descartar
frames, reducir trabajo, aumentar el límite o cambiar el requisito de calidad.

Ejecuta también el ejemplo de medición local y registra únicamente el entorno
en que lo corriste. No compares su resultado con una cámara, un códec, una GPU
o un servidor: el ejemplo mide la construcción y suma del modelo Rust.

La [solución](../../examples/soluciones/capitulo-07.rs) mantiene esa distinción
entre una regla verificable y una cifra local.
