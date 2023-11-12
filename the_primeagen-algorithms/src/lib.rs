pub fn linear_search(haystack: Vec<i32>, needle: i32) -> i32 {
    for i in 0..=haystack.len() {
        if haystack[i] == needle {
            return i as i32;
        }
    }
    return -1;
}

pub fn binary_search(haystack: Vec<i32>, needle: i32) -> i32 {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let value = haystack[mid];
        if value == needle {
            return value as i32;
        } else if needle > value {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    return -1;
}

pub fn two_crystal_ball(breaks: Vec<bool>) -> i32 {
    let length = breaks.len() as f32;
    let jmp = length.sqrt().max(0.0) as usize;

    let mut i = jmp;
    while i < (length as usize) {
        if breaks[i] {
            break;
        }
        i += jmp;
    }

    for j in i - jmp..=i {
        if breaks[j] {
            return j as i32;
        }
    }

    return -1;
}

pub fn bubble_sort(mut haystack: Vec<i32>) -> Vec<i32> {
    let mut range = haystack.len() - 1;
    while range >= 1 {
        let mut swapped = false;
        for i in 0..range {
            if haystack[i] > haystack[i + 1] {
                haystack.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        range -= 1;
    }

    haystack
}
