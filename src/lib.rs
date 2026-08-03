//! Procesamiento de video en streaming para Jeresoft Academy.
//!
//! El crate acompañará el curso `rust-video`. Sus implementaciones partirán de
//! contratos reproducibles de pipeline y orquestación; no reimplementarán
//! códecs, protocolos ni modelos de visión.

#![forbid(unsafe_code)]

/// Declara que el crate base se puede enlazar antes de introducir capítulos.
pub fn course_status() -> &'static str {
    "planned"
}

#[cfg(test)]
mod tests {
    use super::course_status;

    #[test]
    fn crate_base_declares_el_estado_planeado() {
        assert_eq!(course_status(), "planned");
    }
}
