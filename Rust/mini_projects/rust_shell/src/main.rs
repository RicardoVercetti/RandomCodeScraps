// a shell taked stdin for itself and gives the same to other programs too
// shell also takes stdout form other programs that it runs
// cd is built into the shell's current process
// ctrl + c should send the sigint to the process that the shell is running


// other notes

// things like stdin and stdout have to be already built
// calling commands that are available in the operating system bin should be there

use std::io::{ stdin, stdout, Write };
use std::process::Command;

fn main() {
    
    loop{
    // use the `>` charecter as the prompt
    // needs to explicitly flush this to ensure it prints before read_line
    print!("> ");
    stdout().flush();
    
    let mut input = String::new();
    
    stdin().read_line(&mut input).unwrap();
    
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    
    
    let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();
    
    child.wait();
    }
}
