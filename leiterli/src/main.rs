use leiterli::Dice;
use leiterli::Play;
use leiterli::Board;

use clap::App;
use clap::Arg;

fn is_larger_than_zero(v: String) -> Result<(), String> {
    let v_int = v.parse::<u64>().unwrap();
    if v_int > 0 { return Ok(()); }
    Err(String::from("The value is not larger than zero"))
}

fn main() {

    let _matches = App::new("leiterli")
       .version("0.1")
       .about("Simulates the Leiterli game")
       .author("Samuel J. Peter")
       .args(&[
            Arg::with_name("n_games")
                .short("n")
                .long("n_mc")
                .help("number of repeated games")
                .takes_value(true)
                .validator(is_larger_than_zero)
                .default_value("100")
        ])
        .after_help("Longer explanation to appear after the options when \
            displaying the help information from --help or -h")
       .get_matches();

    // number of games to be played
    let n = _matches.value_of("n_games").unwrap().parse::<usize>().unwrap();

    // the board
    let b = Board::new(&[
        (2, 29),
        (5, 21),
        (8, 6),
        (26, 1),
        (32, 31),
        (37, 40),
        (42, 16),
        (44, 43),
        (53, 49),
        (55, 81),
        (60, 95),
        (71, 47),
        (82, 72),
        (85, 110),
        (87, 115),
        (89, 64),
        (90, 117),
        (102, 77),
        (109, 99),
        (122, 113),
        (127, 107)]);

    // the dice to throw
    let mut dice = Dice::new(1,6);

    // vector of games
    let mut plays: Vec<Play> = Vec::new();
    plays.reserve_exact(n);

    // the game is on
    for _i in 0..n {
        let mut p = Play::new(&b);
        while !p.done() {
            p.move_by(dice.throw());
        }
        plays.push(p);
    }

    // do analysis
    plays.sort();
    let n_p = 20;
    println!("{:?} -> {:?}", 0, plays.first().unwrap().moves);
    for i in 1..n_p {
        let p_i = (i as f64)/(n_p as f64);
        println!("{:?} -> {:?}", p_i, plays[(p_i * n as f64) as usize].moves);
    }
    println!("{:?} -> {:?}", 1, plays.last().unwrap().moves);
    
}
