use crate::input::Input;

const MAX_POSITION: u64 = 10;
const SCORE_REQUIRED_PART_2: u8 = 21;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut game = Game::parse(input.text)?;
    if input.is_part_one() {
        let mut die_roll_count = 0_u64;
        let mut p1_score_saved = 0;
        let mut p2_score_saved = 0;
        loop {
            let next_dice_sum = 6 + die_roll_count * 3;
            die_roll_count += 3;
            game.on_dice_sum(next_dice_sum);
            p1_score_saved += u32::from(game.player_1_score);
            game.player_1_score = 0;
            if p1_score_saved >= 1000 {
                return Ok(u64::from(p2_score_saved) * die_roll_count);
            }
            game = game.switch_players();
            std::mem::swap(&mut p1_score_saved, &mut p2_score_saved);
        }
    } else {
        let mut outcome_cache = vec![GameOutcome::default(); Game::MAX_POSSIBLE_STATES];
        let wins = play_game_part_2(game, &mut outcome_cache);
        Ok(std::cmp::max(wins.player_1_wins, wins.player_2_wins))
    }
}

#[derive(Copy, Clone)]
struct Game {
    player_1_position: u8,
    player_2_position: u8,
    player_1_score: u8,
    player_2_score: u8,
}

impl Game {
    const MAX_POSSIBLE_STATES: usize = SCORE_REQUIRED_PART_2 as usize
        * MAX_POSITION as usize
        * SCORE_REQUIRED_PART_2 as usize
        * MAX_POSITION as usize;

    fn parse(text: &str) -> Result<Self, String> {
        fn parse_line(line: Option<&str>) -> Result<u8, String> {
            line.and_then(|s| {
                if s.len() < 28 {
                    return None;
                }
                s[28..].parse::<u8>().ok()
            })
            .ok_or_else(|| "Invalid input".to_string())
        }

        let mut lines = text.lines();
        let player_1_position = parse_line(lines.next())?;
        let player_2_position = parse_line(lines.next())?;
        let valid_positions = 1..=(MAX_POSITION as u8);
        if !(valid_positions.contains(&player_1_position)
            && valid_positions.contains(&player_2_position))
        {
            return Err(format!(
                "Positions must be in the interval [1,{}]",
                MAX_POSITION
            ));
        }
        Ok(Self {
            player_1_position: player_1_position - 1,
            player_2_position: player_2_position - 1,
            player_1_score: 0,
            player_2_score: 0,
        })
    }

    fn on_dice_sum(&mut self, dice_sum: u64) {
        self.player_1_position =
            ((u64::from(self.player_1_position) + dice_sum) % MAX_POSITION) as u8;
        self.player_1_score += self.player_1_position + 1;
    }

    fn unique_hash(self) -> usize {
        (u64::from(self.player_1_score)
            * MAX_POSITION
            * u64::from(SCORE_REQUIRED_PART_2)
            * MAX_POSITION
            + u64::from(self.player_1_position) * u64::from(SCORE_REQUIRED_PART_2) * MAX_POSITION
            + u64::from(self.player_2_score) * MAX_POSITION
            + u64::from(self.player_2_position)) as usize
    }

    const fn switch_players(self) -> Self {
        Self {
            player_1_position: self.player_2_position,
            player_1_score: self.player_2_score,
            player_2_position: self.player_1_position,
            player_2_score: self.player_1_score,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct GameOutcome {
    player_1_wins: u64,
    player_2_wins: u64,
}

fn play_game_part_2(game: Game, outcome_cache: &mut [GameOutcome]) -> GameOutcome {
    if game.player_2_score >= SCORE_REQUIRED_PART_2 {
        return GameOutcome {
            player_1_wins: 0,
            player_2_wins: 1,
        };
    }

    let unique_game_hash = game.unique_hash();
    let cached_outcome = outcome_cache[unique_game_hash];
    if cached_outcome.player_1_wins | cached_outcome.player_2_wins != 0 {
        return cached_outcome;
    }

    let mut computed_outcome = GameOutcome::default();

    for (dice_sum, frequency) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut game_for_roll = game;
        game_for_roll.on_dice_sum(dice_sum);

        let outcomes_for_roll = play_game_part_2(game_for_roll.switch_players(), outcome_cache);
        computed_outcome.player_1_wins += outcomes_for_roll.player_2_wins * frequency;
        computed_outcome.player_2_wins += outcomes_for_roll.player_1_wins * frequency;
    }

    outcome_cache[unique_game_hash] = computed_outcome;
    computed_outcome
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "Player 1 starting position: 4
Player 2 starting position: 8";
    test_part_one!(example => 739_785);
    test_part_two!(example => 444_356_092_776_315);

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => 903_630);
    test_part_two!(real_input => 303_121_579_983_974);
}
