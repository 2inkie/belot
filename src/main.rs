use std::io;
enum UserInput {
    Score(i16),
    NewGame,
}
fn team_score() -> UserInput {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();

        if trimmed_input == "new game" {
            break UserInput::NewGame;
        }

        match trimmed_input.parse::<i16>() {
            Ok(num) => return UserInput::Score(num),
            Err(_) => {
                println!("Invalid input, please enter a number or 'new game'.");
                continue;
            }
        }
    }
}
fn main() {
    println!("Belot, gaz!");

    const MAX_SCORE: i16 = 151;
    loop {
        let mut team_a_score: i16 = 0;
        let mut team_b_score: i16 = 0;

        let mut team_a_wins = false;
        let mut team_b_wins = false;

        while team_a_score < MAX_SCORE || team_b_score < MAX_SCORE {
            println!("Team A score: {}, Team B score: {}", team_a_score, team_b_score);
            println!("Enter the points scored by Team A (or 'new game' to start a new game): ");

            let input_a = team_score();
            match input_a {
                UserInput::NewGame => {
                    println!("Restarting game...");
                    break;
                }
                UserInput::Score(score) => {
                    team_a_score += score;
                    if team_a_score >= MAX_SCORE {
                        team_a_wins = true;
                    }
                }
            }
            
            println!("Enter the points scored by Team B (or 'new game' to start a new game): ");

            let input_b = team_score();
            match input_b {
                UserInput::NewGame => {
                    println!("Restarting game...");
                    break;
                }
                UserInput::Score(score) => {
                    team_b_score += score;
                    if team_b_score >= MAX_SCORE {
                        team_b_wins = true;
                    }
                }
            }

            if team_a_wins || team_b_wins {
                println!("Final scores - Team A: {}, Team B: {}", team_a_score, team_b_score);
                break;
            }

            println!("---------------------------------------------------------");
        }
    }
}