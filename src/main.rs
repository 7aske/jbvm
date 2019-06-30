pub mod vm;
pub mod repl;
pub mod instruction;

use repl::REPL;

fn main() {
    // code to add 500 to 255
    //  let code = vec![1, 0, 0, 12, 7, 0, 0, 0, 0, 0, 0, 0, 200, 0];
    //  let mut virt_machine = VM::new();
    //  virt_machine.set_code(&code);
    //  virt_machine.run();
    let mut repl = REPL::new();
    repl.run();
}
