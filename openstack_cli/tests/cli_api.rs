mod cli {
    mod api {
        use assert_cmd::prelude::*;
        use std::process::Command;

        #[test]
        #[ignore]
        fn cli_api() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("api").arg("network").arg(".");
            cmd.assert().success();

            Ok(())
        }
    }
}
