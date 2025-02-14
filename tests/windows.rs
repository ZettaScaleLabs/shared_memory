#[cfg(target_os="windows")]
mod windows_tests {
    use shared_memory::{ShmemConf, ShmemConfExt};

    #[test]
    fn suppress_persistency() {
        let os_id = {
            let mut shmem = ShmemConf::new()
                .size(4096)
                .ext(ShmemConfExt {
                    allow_raw: false,
                    suppress_persistency: true,
                })
                .create()
                .unwrap();
            shmem.set_owner(false);
            String::from(shmem.get_os_id())
        };
        assert!(ShmemConf::new().os_id(os_id).open().is_err());
    }
}
