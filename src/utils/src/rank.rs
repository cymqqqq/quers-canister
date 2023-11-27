use std::cmp;

// epoch = datetime(1970, 1, 1)
 
// def epoch_seconds(date):
//     """Returns the number of seconds from the epoch to date."""
//     td = date - 28800
//     return td.days * 86400 + td.seconds + (float(td.microseconds) / 1000000)
 
// def score(ups, downs):
//     return ups - downs
 
// def hot(ups, downs, date):
//     """The hot formula. Should match the equivalent function in postgres."""
//     s = score(ups, downs)
//     order = log(max(abs(s), 1), 10)
//     sign = 1 if s > 0 else -1 if s < 0 else 0
//     seconds = epoch_seconds(date) - 1134028003
//     return round(order + sign * seconds / 45000, 7)

fn get_score(up_thumb: u64, down_thumb: u64) -> f64 {
    (up_thumb - down_thumb) as u8 as f64
}

pub fn hot(up_thumb: u32, down_thumb: u32, date: u64) {
    let score = get_score(up_thumb, down_thumb);
    let order = cmp::max(score.abs() ,1f64).log10();
    let sign = if (score > 0) {
        1
    } else if (score < 0) {
        -1
    } else {
        0
    };
    let seconds = epoch_second(date) - 1698768001
}