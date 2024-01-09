mod cli {
    mod compute {
        use assert_cmd::prelude::*;
        use std::process::Command;

        #[test]
        #[ignore]
        fn extension_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("compute").arg("extension").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn flavor_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("compute").arg("flavor").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn keypair_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("compute").arg("keypair").arg("list");
            cmd.assert().success();

            Ok(())
        }
    }
}
