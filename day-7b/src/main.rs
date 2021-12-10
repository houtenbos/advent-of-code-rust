use std::fs::read_to_string;

fn main() {
    let crabs: Vec<i32> = read_to_string("input.txt")
                                                        .unwrap()
                                                        .split(",")
                                                        .map(|n| n.parse().unwrap())
                                                        .collect();

    let first: i32 = *crabs.iter().min().unwrap();
    let last: i32 = *crabs.iter().max().unwrap();
    let mut min_fuel_costs = 1000000000;
    let mut cheapest_position = 0;

    for position in first..last {
        let fuel_costs = calculate_fuel_costs(&position, &crabs);
        if fuel_costs < min_fuel_costs {
            min_fuel_costs = fuel_costs;
            cheapest_position = position;
        }
        println!("costs {} at position {}", fuel_costs, position);
    }
    println!("costs {} at position {}", min_fuel_costs, cheapest_position);

}

fn calculate_fuel_costs(p: &i32, crabs: &Vec<i32>) -> i32 {
    let mut costs = 0;
    for crab in crabs {
        let n = (crab-p).abs();
        costs += n * (n + 1) / 2;
    }
    return costs;
}
