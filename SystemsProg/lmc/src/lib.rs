/// An instruction in the LMC is represented by a three digits number. The first
/// digit being the `code`, and the other digits are the index of the mailbox to
/// apply the instruction. Notice that we cast the index from `i32` to `usize`
/// with `try_from`.
pub fn compute(mut mailbox: [i32; 100]) {
    let mut program_cnt: usize = 0;
    let mut accumulator: i32 = 0;

    let instruction = mailbox[program_cnt];
    let code: i32 = instruction / 100;
    let index: usize = usize::try_from(instruction % 100).unwrap();

    match code {
        1 => accumulator += mailbox[index], // ADD
        2 => accumulator -= mailbox[index], // SUBSTRACT
        3 => mailbox[index] = accumulator,  // STORE
        5 => accumulator = mailbox[index],  // LOAD
        0 => return,                        // HALT
        _ => panic!("{}", format!("Invalid action: {}", mailbox[program_cnt]))
    }
}
