use std::io::Write;

mod solver;
use solver::Solver;

fn main() -> std::io::Result<()> {
    let mut solver = Solver::new();
    let mut user_input = String::new();
    loop {
        print!("Enter prompt: ");
        std::io::stdout().flush().unwrap();
        // Clear the terminal and the user input buffer
        print!("\x1B[2J\x1B[1;1H");
        user_input.clear();
        std::io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_lowercase();

        let (word_list, punctuated_word_list) = solver.solve_prompt(user_input.clone());
        for word in word_list {
            println!("{}", word);
        }
        for word in punctuated_word_list {
            println!("{}", word);
        }
    }
}
