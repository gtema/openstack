mod cli {
    mod block_storage {
        use assert_cmd::prelude::*;
        use std::process::Command;

        #[test]
        #[ignore]
        fn volume_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("block-storage").arg("volume").arg("list");
            cmd.assert().success();

            Ok(())
        }
    }
}
