/*
    https://www.codewars.com/kata/545af3d185166a3dec001190/train/rust
 */
fn main() {
    println!("{:?}",each_cons(&[1, 2, 3, 4], 2));
    println!("{:?}",each_cons(&[3, 5, 8, 13], 1));
}

fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    let mut res:Vec<Vec<u8>>= Vec::new();
    for i in 0..arr.len() {
        let mut tmp: Vec<u8> = Vec::new();
        if i + n > arr.len() {
            break
        }
        for j in i..i + n {
            tmp.push(arr[j]);
            if j + 1 == arr.len() {
                break
            }
        }
        res.push(tmp);
    }
    res
}