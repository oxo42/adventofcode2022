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

    pub fn what_beats(&self) -> Self {
        use Hand::*;
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    elf_hand: Hand,
    my_hand: Hand,
    needed_result: GameResult,
}

impl Game {
    pub fn new(elf_hand: Hand, my_hand: Hand) -> Self {
        Self {
            elf_hand,
            my_hand,
            needed_result: GameResult::Win,
        }
    }

    pub fn from_chars(first: char, second: char) -> Self {
        let elf_hand = match first {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!("unsupported char {first:}"),
        };
        let my_hand = match second {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("unsupported char {second:}"),
        };

        let needed_result = match second {
            'X' => GameResult::Loss,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("unsupported char {second:}"),
        };

        Self {
            elf_hand,
            my_hand,
            needed_result,
        }
    }

    fn my_result(&self) -> GameResult {
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

    fn part2_my_hand(&self) -> Hand {
        let beats_elf = self.elf_hand.what_beats();
        match self.needed_result {
            GameResult::Win => self.elf_hand.what_beats(),
            GameResult::Draw => self.elf_hand,
            GameResult::Loss => self.elf_hand.beats(),
        }
    }

    pub fn part2_score(&self) -> i64 {
        (self.needed_result as i64) + (self.part2_my_hand() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_scores() {
        assert_eq!(Game::from_chars('A', 'Y').part2_score(), 4);
        assert_eq!(Game::from_chars('B', 'X').part2_score(), 1);
        assert_eq!(Game::from_chars('C', 'Z').part2_score(), 7);
    }

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
