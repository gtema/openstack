mod cli {
    mod network {
        use assert_cmd::prelude::*;
        use std::process::Command;

        #[test]
        #[ignore]
        fn extension_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("network").arg("extension").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn az_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("network").arg("availability-zone").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn network_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("network").arg("network").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn port_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("network").arg("port").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn subnet_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("network").arg("subnet").arg("list");
            cmd.assert().success();

            Ok(())
        }
    }
}
