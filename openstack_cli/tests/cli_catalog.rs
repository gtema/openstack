mod cli {
    mod catalog {
        use assert_cmd::prelude::*;
        use std::process::Command;

        #[test]
        #[ignore]
        fn cli_catalog() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("catalog").arg("list");
            cmd.assert().success();

            Ok(())
        }
    }
}
