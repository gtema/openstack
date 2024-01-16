mod cli {
    mod identity {
        use assert_cmd::prelude::*;
        use std::process::Command;

        // TODO(gtema): devstack regular user (the one we want to
        // use for regular tests) has no permission to list
        // project
        // #[test]
        // #[ignore]
        // fn project_list() -> Result<(), Box<dyn std::error::Error>> {
        //     let mut cmd = Command::cargo_bin("osc")?;

        //     cmd.arg("identity").arg("project").arg("list");
        //     cmd.assert().success();

        //     Ok(())
        // }
    }
}
