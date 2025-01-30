fn find_missing(seq: &[i32]) -> i32 {
    let start = if seq[0] < seq[seq.len() - 1] {seq[0]} else {seq[seq.len() - 1]};
    let stop = if seq[0] > seq[seq.len() - 1] {seq[0]} else {seq[seq.len() - 1]};
    let diff = (stop - start) / seq.len() as i32;
    for i in (start..stop).step_by(diff as usize) {
        if !seq.contains(&i) {
            return i
        }
    }
    seq[seq.len() - 2]
}

fn find_missing_best(seq: &[i32]) -> i32 {
    let diff = ((seq.last().unwrap() + seq.first().unwrap()) * (seq.len() as i32 + 1)) / 2;
    diff - seq.iter().sum::<i32>()
}

fn main() {
    println!("{}", find_missing(&[862063, 862013, 861963, 861913, 861863, 861813, 861763, 861713, 861663, 861613, 861563, 861513, 861463, 861413, 861363, 861313, 861213, 861163, 861113, 861063, 861013, 860963, 860913, 860863, 860813, 860763, 860713, 860663, 860613, 860563, 860513, 860463]));
    println!("{}", find_missing_best(&[862063, 862013, 861963, 861913, 861863, 861813, 861763, 861713, 861663, 861613, 861563, 861513, 861463, 861413, 861363, 861313, 861213, 861163, 861113, 861063, 861013, 860963, 860913, 860863, 860813, 860763, 860713, 860663, 860613, 860563, 860513, 860463]));
}
