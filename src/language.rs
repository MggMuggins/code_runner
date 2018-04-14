use std::fs::File;
use std::io::Write;
use std::process::Command;

use failure::Error;

use DOCKER_DIR;

pub trait Language {
    fn run(&self) -> Result<String, Error>;
}


pub struct Python {
    code: String
}

impl Python {
    pub fn new(code: String) -> Python {
        Python { code }
    }
}

impl Language for Python {
    fn run(&self) -> Result<String, Error> {
        let working_docker_dir = &(DOCKER_DIR.to_owned() + "/python/");
        
        // Write the code to the file to be built into the container
        let filename = working_docker_dir.to_string() + "/code.py";
        let mut file = File::create(&filename)?;
        file.write(self.code.as_bytes());
        
        let _build_output = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg("python_runner")
            .arg(working_docker_dir)
            .output()?;
        
        let output = Command::new("docker")
            .arg("run")
            .arg("python_runner")
            .output()?;
        
        Ok("Stdout:```\n".to_owned()
            + &String::from_utf8(output.stdout)?
            + &"\n```\nStderr:```\n".to_owned()
            + &String::from_utf8(output.stderr)?
            + &"\n```")
    }
}
