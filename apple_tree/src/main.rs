use clap::App;
use clap::Arg;

fn is_larger_than_zero(v: String) -> Result<(), String> {
    let v_int = v.parse::<u64>().unwrap();
    if v_int > 0 { return Ok(()); }
    Err(String::from("The value is not larger than zero"))
}

fn main() {
    let _matches = App::new("apple_tree")
       .version("0.1")
       .about("Simulates the Apple Tree game")
       .author("Samuel J. Peter")
       .args(&[
            Arg::with_name("n_players")
                .short("p")
                .long("players")
                .help("number of players in the game")
                .takes_value(true)
                .validator(is_larger_than_zero)
                .default_value("4"),
            Arg::with_name("n_games")
                .short("n")
                .long("n_mc")
                .help("number of repeated games")
                .takes_value(true)
                .validator(is_larger_than_zero)
                .default_value("1000"),
            Arg::with_name("n_apples")
                .short("a")
                .long("apples")
                .help("number of apples per tree")
                .takes_value(true)
                .validator(is_larger_than_zero)
                .default_value("10")
        ])
        .after_help("Longer explanation to appear after the options when \
            displaying the help information from --help or -h")
       .get_matches();

    let _n_apples = _matches.value_of("n_apples").unwrap().parse::<u64>().unwrap();
    let _n_players = _matches.value_of("n_players").unwrap().parse::<u64>().unwrap();
    let _n_games = _matches.value_of("n_games").unwrap().parse::<u64>().unwrap();
}


