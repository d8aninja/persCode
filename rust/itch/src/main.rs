// use mdbook
// explain snowflake use cases
// copyleft, permissive, cc licenses
// approachability

// muh itch
// takes a repo name, postpends it to a git base path A
// gets the branch name B
// get number of commits since last merge B
// prints A:B, C

use regex::Regex;
use std::{
//    intrinsics::panic_if_uninhabited,
    io,
    process::{
        Command, Stdio
    }
};

fn main() -> Result<(), io::Error> {
    let path = String::from("/Users/jeffvangeete/git/");
    let mut repo = String::new();

    println!("Please enter a repo name, below, and hit the Enter key...");

    io::stdin().read_line(&mut repo).expect("Failed to understand your response.");

    // ifconfig
    if false {
        let ifconf_output = Command::new("ifconfig").output()?;

        println!("ifconfig output: \n\n {:?}", ifconf_output);
    }

    // git_summary
    if false {
        let git_summary = Command::new("~/git-status ~/git").output()?;

        println!("{:?}", git_summary);
    }

    if false {
        let output = Command::new("git").arg("log").arg("--oneline").output().unwrap();

        if !output.status.success() {
            panic!("Command executed with failing error code");
        }

        let pattern = Regex::new(r"(?x)
                                   ([0-9a-fA-F]+) # commit hash
                                   (.*)           # The commit message"
        ).unwrap();

        println!("{:?} \n\n {:?}", output.stdout, pattern);
    }

    if true {
        let directory = std::env::current_dir()?;
        let mut du_output_child = Command::new("du")
            .arg("-ah")
            .arg(&directory)
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(du_output) = du_output_child.stdout.take() {
            let mut sort_output_child = Command::new("sort")
                .arg("-hr")
                .stdin(du_output)
                .stdout(Stdio::piped())
                .spawn()?;

            du_output_child.wait()?;

            if let Some(sort_output) = sort_output_child.stdout.take() {
                let head_output_child = Command::new("head")
                    .args(&["-n", "10"])
                    .stdin(sort_output)
                    .stdout(Stdio::piped())
                    .spawn()?;

                let head_stdout = head_output_child.wait_with_output()?;

                sort_output_child.wait()?;

                println!(
                    "Top 10 biggest files and directories in '{}':\n{}",
                    directory.display(),
                    String::from_utf8(head_stdout.stdout).unwrap()
                );
            }
        }
    }

    Ok(())
}