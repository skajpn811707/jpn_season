use rand::{thread_rng, Rng};

enum Color {
    PINK,
    GREEN,
    RED,
    WHITE
}

struct JapanSeason {
    color_name: String,
    season_name: String,
}

impl JapanSeason {
    fn get_info(color: &str, season: &str) -> Self {
        JapanSeason {
            color_name: String::from(color),
            season_name: String::from(season)
        }
    }
 }

fn main() {
    let mut rng = thread_rng();
    let random_color = rng.gen_range(0..=3);

    let color_index = match random_color {
        0 => Color::PINK,
        1 => Color::GREEN,
        2 => Color::RED,
        3 => Color::WHITE,
        _ => unreachable!()
    };

    let (japan_season_color, japan_season_name) = match color_index {
        Color::PINK => ("pink", "spring"),
        Color::GREEN => ("green", "summer"),
        Color::RED => ("red", "autumn"),
        Color::WHITE => ("white", "winter")
    };

    let japan_season = JapanSeason::get_info(japan_season_color, japan_season_name);

    println!("Japan season is {} and his color is {}", japan_season.season_name, japan_season.color_name);
}