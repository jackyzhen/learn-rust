use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats {
    mean: f32,
    mode: i32,
    median: f32,
}

pub fn num_stats(list: &mut Vec<i32>) -> Stats {
    list.sort_unstable();
    let len = list.len();
    let sum: i32 = list.iter().sum();
    let mut h: HashMap<i32, i32> = HashMap::new();
    for i in list.iter() {
        let c = h.entry(*i).or_insert(0);
        *c += 1;
    }

    let median = match len % 2 == 0 {
        true => (list[len / 2 - 1] + list[len / 2] / 2) as f32 / 2.0,
        false => list[len / 2] as f32,
    };

    let mode = {
        let mut max = 0;
        let mut max_i = 0;
        for (i, c) in h {
            if max < c {
                max = c;
                max_i = i;
            }
        }
        max_i
    };

    Stats {
        mean: sum as f32 / len as f32,
        mode,
        median,
    }
}

#[cfg(test)]
mod tests {
    use stats::num_stats;

    #[test]
    fn it_calculates_mean() {
        let stats = num_stats(&mut vec![10, 2, 38, 23, 38, 23, 21]);
        assert_eq!(22.142857142857, stats.mean);
    }

    #[test]
    fn it_calculates_mode() {
        let stats = num_stats(&mut vec![10, 2, 38, 23, 38, 23, 21]);
        assert_eq!(23, stats.mode);
    }

    #[test]
    fn it_calculates_median() {
        let stats = num_stats(&mut vec![10, 2, 38, 23, 38, 23, 21]);
        assert_eq!(23.0, stats.median);
    }
}
