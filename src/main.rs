use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Write;

struct Point {
    x: f64,
    y: f64
}

impl Point {
    // エッジの距離を求める
    fn distance(&self, point: &Point) -> f64 {
        let x: f64 = point.x - self.x;
        let y: f64 = point.y - self.y;
        return (x * x + y * y).sqrt();
    }
}

// エッジの総合距離を求める
fn sum_distance(points: &Vec<Point>, p_indexes: &Vec<usize>) -> f64 {
    let mut sum_dist: f64 = 0.;
    for i in 0..p_indexes.len() - 1 {
        let dist = points[p_indexes[i]].distance(&points[p_indexes[i + 1]]);
        sum_dist += dist;
    }
    return sum_dist;
}

fn main() {
    let n: i32 = 100;
    let n_iter: i32 = 200000;
    
    let mut rng = thread_rng();
    let mut p_indexes: Vec<usize> = (0..n.try_into().unwrap()).collect();
    let mut dists: Vec<f64> = Vec::with_capacity(n_iter.try_into().unwrap());
    let mut sum_dist: f64 = 0.0;
    let mut points: Vec<Point> = Vec::with_capacity(n.try_into().unwrap());

    for _i in 0..n {
        let rand1: f64 = rng.gen_range(0.0..10000.0);
        let rand2: f64 = rng.gen_range(0.0..10000.0);
        points.push(Point {x: rand1, y: rand2});
    }

    for _i in 0..n_iter {
        let rand1: usize = rng.gen_range(0..n).try_into().unwrap();
        let rand2: usize = rng.gen_range(0..n).try_into().unwrap();
        (p_indexes[rand1], p_indexes[rand2]) = (p_indexes[rand2], p_indexes[rand1]);
        let sum_tmp_dist: f64 = sum_distance(&points, &p_indexes);

        if sum_tmp_dist < sum_dist || sum_dist == 0.0 {
            sum_dist = sum_tmp_dist;
        } else {
            (p_indexes[rand1], p_indexes[rand2]) = (p_indexes[rand2], p_indexes[rand1]);
        }
        dists.push(sum_dist);
    }

    let mut file = File::create("2-opt.txt").unwrap();
    for i in 0..n_iter{
        let idx: usize = i as usize;
        let s: String = format!("{}\n", dists[idx]);
        let _ = file.write(s.as_bytes());
    }

    println!("{}", sum_dist);
}
