pub fn knap_sack(mut capacity: f32, profit_weight: &Vec<(f32, f32)>) -> f32 {
    let mut pw = profit_weight
        .iter()
        .map(|&(profit, weight)| (profit, weight, profit / weight))
        .collect::<Vec<(f32, f32, f32)>>();

    // sort by profit / unit wight
    pw.sort_by(|&a, &b| a.2.partial_cmp(&b.2).unwrap());

    let mut profit = 0.;

    while capacity > 0. && !pw.is_empty() {
        // if capacity > obj weight that take fraction of it.
        // calculate profit: weight * (profit/unit_weight) than add to profit
        let obj = pw.pop().unwrap();
        let mn_weight = if capacity < obj.1 { capacity } else { obj.1 };

        capacity -= mn_weight;
        profit += mn_weight * obj.2;
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let pw: Vec<(f32, f32)> = vec![
            (10., 2.),
            (5., 3.),
            (15., 5.),
            (7., 7.),
            (6., 1.),
            (18., 4.),
            (3., 1.),
        ];
        assert_eq!(55.333332, knap_sack(15., &pw))
    }
}
