pub mod sited_io {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("./FILE_DESCRIPTOR_SET");

    pub mod report {
        pub mod v1 {
            include!("sited_io.report.v1.rs");
        }
    }
}
