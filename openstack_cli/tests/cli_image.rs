mod cli {
    mod image {
        use assert_cmd::prelude::*;
        use std::process::Command;

        #[test]
        #[ignore]
        fn image_list() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("image").arg("list");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_image_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("image").arg("show");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_images_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("images").arg("show");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_member_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("member").arg("show");
            cmd.assert().success();

            Ok(())
        }

        #[test]
        #[ignore]
        fn schema_members_show() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin("osc")?;

            cmd.arg("image").arg("schema").arg("members").arg("show");
            cmd.assert().success();

            Ok(())
        }
    }
}
