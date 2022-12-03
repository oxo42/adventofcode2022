#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    pub fn beats(&self) -> Self {
        use Hand::*;
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    elf_hand: Hand,
    my_hand: Hand,
}

impl Game {
    pub fn new(elf_hand: Hand, my_hand: Hand) -> Self {
        Self { elf_hand, my_hand }
    }

    pub fn from_chars(elf: char, me: char) -> Self {
        let elf_hand = match elf {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!("unsupported char {elf:}"),
        };
        let my_hand = match me {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("unsupported char {me:}"),
        };

        Self::new(elf_hand, my_hand)
    }

    pub fn my_result(&self) -> GameResult {
        let elf_beats = self.elf_hand.beats();
        let my_beats = self.my_hand.beats();
        match (elf_beats, my_beats) {
            _ if my_beats == self.elf_hand => GameResult::Win,
            _ if elf_beats == self.my_hand => GameResult::Loss,
            _ => GameResult::Draw,
        }
    }

    pub fn score(&self) -> i64 {
        (self.my_result() as i64) + (self.my_hand as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scores() {
        assert_eq!(Game::new(Hand::Rock, Hand::Rock).score(), 4);
        assert_eq!(Game::new(Hand::Rock, Hand::Paper).score(), 8);
        assert_eq!(Game::new(Hand::Rock, Hand::Scissors).score(), 3);
    }

    #[test]
    fn test_wins() {
        assert_eq!(
            Game::new(Hand::Rock, Hand::Rock).my_result(),
            GameResult::Draw
        );
        assert_eq!(
            Game::new(Hand::Rock, Hand::Paper).my_result(),
            GameResult::Win
        );
        assert_eq!(
            Game::new(Hand::Rock, Hand::Scissors).my_result(),
            GameResult::Loss
        );

        assert_eq!(
            Game::new(Hand::Paper, Hand::Rock).my_result(),
            GameResult::Loss
        );
        assert_eq!(
            Game::new(Hand::Paper, Hand::Paper).my_result(),
            GameResult::Draw
        );
        assert_eq!(
            Game::new(Hand::Paper, Hand::Scissors).my_result(),
            GameResult::Win
        );

        assert_eq!(
            Game::new(Hand::Scissors, Hand::Rock).my_result(),
            GameResult::Win
        );
        assert_eq!(
            Game::new(Hand::Scissors, Hand::Paper).my_result(),
            GameResult::Loss
        );
        assert_eq!(
            Game::new(Hand::Scissors, Hand::Scissors).my_result(),
            GameResult::Draw
        );
    }

    #[test]
    fn test_score() {}
}
