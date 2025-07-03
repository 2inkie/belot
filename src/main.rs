use std::io;
fn main() {
    println!("Belot, gaz!");

    let max_score = 151;
    loop {
        let mut team_a_score: i16 = 0;
        let mut team_b_score: i16 = 0;

        let mut team_a_wins = false;
        let mut team_b_wins = false;

        while team_a_score < max_score || team_b_score < max_score {
            println!("Team A score: {}, Team B score: {}", team_a_score, team_b_score);
            println!("Enter the points scored by Team A (or 'new game' to start a new game): ");

            let mut input_a = String::new();
            io::stdin().read_line(&mut input_a).expect("Failed to read line");
            let input = input_a.trim();
            if input == "new game" {
                break;
            }

            match input.parse::<i16>() {
                Ok(points) => {
                    team_a_score += points;
                    if team_a_score >= max_score {
                        team_a_wins = true;
                    }
                },
                
                Err(_) => {
                    println!("Invalid input, please enter a number or 'new game'.");
                    continue;
                }
            }
            
            println!("Enter the points scored by Team B (or 'new game' to start a new game): ");
            let mut input_b = String::new();
            io::stdin().read_line(&mut input_b).expect("Failed to read line");
            let input = input_b.trim();

            match input.parse::<i16>() {
                Ok(points) => {
                    team_b_score += points;
                    if team_b_score >= max_score {
                        team_b_wins = true;
                    }
                },
                
                Err(_) => {
                    println!("Invalid input, please enter a number or 'new game'.");
                    continue;
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