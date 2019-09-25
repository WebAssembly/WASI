pub mod unstable {
    use witx;
    pub fn preview0() -> witx::Document {
        witx::parse(include_str!(concat!(
            env!("OUT_DIR"),
            "/wasi_unstable_preview0.witx"
        )))
        .expect("parses")
    }

    #[cfg(test)]
    #[test]
    fn preview0_works() {
        let packaged = preview0();
        let canon = witx::load("../../phases/unstable/witx/wasi_unstable_preview0.witx")
            .expect("load canonical");
        assert_eq!(packaged, canon);
    }
}
