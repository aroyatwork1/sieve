pub fn get_largest_prime(n: usize) -> Option<usize> {
    // Set an upper bound for seiving.
    let size_sqrt: usize = (n as f64).sqrt().ceil() as usize;
    let mut nums: Vec<usize> = vec![0; n];

    sieve(&mut nums, n, size_sqrt);

    let mut vec: Vec<usize> = nums.iter().enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr != 0 {Some(pr)} else {None} )
        .collect();

    return vec.pop();
}

fn sieve(nums: &mut [usize], size: usize, size_sqrt: usize) {
    for i in 0..size {
        nums[i] = i;
    }

    for i in 0..size {
        let num = nums[i];
        if num < 2 {
            continue;
        }

        if num > size_sqrt {
            break;
        }

        for x in (num.pow(2)..size).step_by(num) {
            nums[x] = 0;
        }
    }
}
