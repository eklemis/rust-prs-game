extern crate fastrand;
extern crate clearscreen;

use std::io::Write;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum MatchResult{
    HumanWin,
    Draw,
    HumanLost
}
#[derive(Debug)]
pub enum PlayerType{
    Human,
    Computer
}
pub struct Player{
    p_type: PlayerType,
    score: i32
}
impl Player {
    pub fn new(p_type: PlayerType)->Player{
        Player { p_type, score: 0 }
    }
    pub fn win(&mut self){
        self.score += 5;
    }
    pub fn curr_score(&self)->i32{
        self.score
    }
}
#[derive(Debug, PartialEq)]
pub enum ChoiceOption {
    Paper,
    Rock,
    Scissors,
    None
}
#[derive(Debug)]
pub struct Choice{
    pub choice: ChoiceOption
}
impl Choice {
    fn from(choice_num: u8)->Choice{
        let choice = match choice_num {
            1 => ChoiceOption::Paper,
            2 => ChoiceOption::Rock,
            3 => ChoiceOption::Scissors,
            _=> ChoiceOption::None
        };
        Choice{
            choice : choice            
        }
    }
    fn beat(&self, opponent_choice: &Choice)->MatchResult{
        if (self.choice == ChoiceOption::Paper && opponent_choice.choice == ChoiceOption::Rock) ||
        (self.choice == ChoiceOption::Rock && opponent_choice.choice == ChoiceOption::Scissors) ||
        (self.choice == ChoiceOption::Scissors && opponent_choice.choice == ChoiceOption::Paper){
            return MatchResult::HumanWin;
        }
        else if self.choice == opponent_choice.choice {
            return MatchResult::Draw;
        }
        MatchResult::HumanLost
    }
}
pub struct Game{
    p1: Player,
    p2: Player
}
impl Game{
    pub fn new()->Game{
        Game{
            p1: Player::new(PlayerType::Human),
            p2: Player::new(PlayerType::Computer)
        }
    }
    pub fn run(&mut self){
        clearscreen::clear().unwrap();
        let mut keep_run = true;
        while keep_run {
            self.welcoming();
            self.run_match();
            
            let mut buffer = String::new();
            print!("Go next Match?[y/n]: ");
            std::io::stdout().flush().unwrap();

            let _buff_info = std::io::stdin().read_line(&mut buffer).unwrap();

            clearscreen::clear().unwrap();
            keep_run = buffer.replace("\n", "") == "y";
        }
    }
    fn run_match(&mut self){
        let user_choice = self.get_user_choice();
        print!("You choose {:?}, ", user_choice.choice);
        std::io::stdout().flush().unwrap();

        let comp_choice = self.get_comp_choice();
        println!("Computer choose {:?}", comp_choice.choice);

        if user_choice.beat(&comp_choice) == MatchResult::HumanWin{
            self.p1.win();
            println!("You WIN");
        }
        else if user_choice.beat(&comp_choice) == MatchResult::HumanLost {
            self.p2.win();
            println!("Computer WIN");
        }
        else{
            println!("DRAW");
        }

        println!("\n\nCURRENT SCORE:");
        println!("You: {}, Computer:{}\n", self.p1.curr_score(), self.p2.curr_score());

    }
    fn welcoming(&self){
        println!("\nWelcome to PAPER, ROCK, SCISSORS Game!");
        println!("Here you'll try to beat computer! Choose Wisely!\n");
    }
    fn get_user_choice(&self)->Choice{
        println!("1. Paper");
        println!("2. Rock");
        println!("3. Scissors");
        let mut buffer = String::new();
        print!("What is your choice: ");
        std::io::stdout().flush().unwrap();
        let _buff_info = std::io::stdin().read_line(&mut buffer).unwrap();
        Choice::from(buffer.replace("\n", "").parse().unwrap())
    }
    
    fn get_comp_choice(&self)->Choice{
        let options: Vec<u8> = vec![1,2,3];
        Choice::from(options[fastrand::usize(..options.len())])
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paper_beats_rock(){
        let pc = Choice::from(1);
        let rc = Choice::from(2);
        assert_eq!(MatchResult::HumanWin, pc.beat(&rc));
    }
    #[test]
    fn rock_lost_paper(){
        let pc = Choice::from(1);
        let rc = Choice::from(2);
        assert_eq!(MatchResult::HumanLost, rc.beat(&pc));        
    }
    #[test]
    fn rock_beats_scissors(){
        let rc = Choice::from(2);
        let sc = Choice::from(3);
        assert_eq!(MatchResult::HumanWin, rc.beat(&sc));
    }
    #[test]
    fn scissors_lost_rock(){
        let rc = Choice::from(2);
        let sc = Choice::from(3);
        assert_eq!(MatchResult::HumanLost, sc.beat(&rc));
    }
    #[test]
    fn scissors_beat_paper(){
        let sc = Choice::from(3);
        let pc = Choice::from(1);
        assert_eq!(MatchResult::HumanWin, sc.beat(&pc));
    }
    #[test]
    fn paper_lost_scissors(){
        let sc = Choice::from(3);
        let pc = Choice::from(1);
        assert_eq!(MatchResult::HumanLost, pc.beat(&sc));
    }
    #[test]
    fn papers_draw(){
        let pc1 = Choice::from(1);
        let pc2 = Choice::from(1);
        assert_eq!(MatchResult::Draw, pc1.beat(&pc2));
    }
    #[test]
    fn rocks_draw(){
        let pc1 = Choice::from(2);
        let pc2 = Choice::from(2);
        assert_eq!(MatchResult::Draw, pc1.beat(&pc2));
    }
    #[test]
    fn scissors_draw(){
        let pc1 = Choice::from(3);
        let pc2 = Choice::from(3);
        assert_eq!(MatchResult::Draw, pc1.beat(&pc2));
    }
}