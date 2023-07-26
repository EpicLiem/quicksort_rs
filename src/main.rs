use std::time::{Duration, Instant};
use rand::{Rng, RngCore};
use csv;
use linreg::{linear_regression, linear_regression_of};

fn partition(high: i64, low: i64, array: &mut [i64]) -> i64 {
    let pivot: i64 = array[high as usize];
    let mut greater = low - 1;

    for i in low..high {
        if array[i as usize] <= pivot {
            greater = greater + 1;
            array.swap(greater as usize, i as usize);
        }
    }
    array.swap((greater + 1) as usize, high as usize);
    return greater + 1;
}

fn quicksort(high: i64, low: i64, array: &mut [i64]) {
    if low < high {
        let pi = partition(high, low, array);
        quicksort(pi - 1, low, array);
        quicksort(high, pi + 1, array);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut data = [(Duration::default(), i64::default()); 1001];
    for i in (0..1000001).step_by(1000) {
        let mut array = vec![0;i];
        let mut trials = [Duration::default();100];
        for k in 0..100 {
            for j in 1..i {
                array[j] = rng.gen_range(i64::MIN..i64::MAX) as i64
            }
            let N: i64 = array.len() as i64;
            let start = Instant::now();
            quicksort(N - 1, 0, &mut array);
            let duration = start.elapsed();
            for j in 1..array.len() {
                assert!(array[j] >= array[j - 1]);
            }
            trials[k] = duration;
        }
        let mut average = Duration::default();
        for k in trials {
            average = average.saturating_add(k);
        }
        println!("{:?}", average);
        average = average.div_f64(100.0);
        println!("Time elapsed in {} length array is: {:?} on average", i, average);
        data[(i / 1000) as usize] = (average, i as i64)
    }
    let mut wtr = csv::Writer::from_path("data.csv").unwrap();
    let mut xs: Vec<f64> = vec![];
    let mut ys : Vec<f64> = vec![];
    for i in data {
        wtr.write_record(&[i.1.to_string(), i.0.as_nanos().to_string()]);
        xs.push(i.1 as f64);
        ys.push(i.0.as_nanos() as f64)
    }
    wtr.flush().unwrap();

    let line: (f64, f64) = linear_regression(&xs,&ys).unwrap();
}
