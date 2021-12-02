fn main() {
    let nums: Vec<i32> = include_str!("../../input/day1.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();
    // count number of increases
    let n_increases = nums
        .windows(2)
        .map(|nums| nums[1] - nums[0])
        .filter(|n| n > &0)
        .count();
    println!("{}", n_increases);
    // count number of increases in sliding window
    let mut base = 0;
    let mut ret = 0;
    while base + 3 < nums.len() {
        ret += if nums[base + 3] - nums[base] > 0 {
            1
        } else {
            0
        };
        base += 1;
    }

    println!("{}", ret);
}
