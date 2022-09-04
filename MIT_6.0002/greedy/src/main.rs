mod food;
use food::Food;
use std::cmp::Ordering;

fn main() {
    let names = vec![
        "wine", "beer", "pizza", "burger", "fries", "cola", "apple", "donut", "cake",
    ];
    let values = vec![89., 90., 95., 100., 90., 79., 50., 10.];
    let calories = vec![123., 154., 258., 354., 365., 150., 95., 195.];

    let mut foods = Food::build_menu(&names, &values, &calories);
    println!("The foods on menu");
    for f in foods.iter() {
        println!("  {}", f);
    }
    println!();

    test_greedys(&mut foods, 750.);
    test_greedys(&mut foods, 800.);
    test_greedys(&mut foods, 1000.);
}

type CompFunc = dyn Fn(&Food, &Food) -> Ordering;

fn test_greedy(foods: &mut [Food], constraint: f64, comp_func: &CompFunc) {
    foods.sort_by(comp_func);
    let (taken, val) = greedy(foods, constraint);
    println!("Total value of items taken = {}", val);
    for f in taken.iter() {
        println!("  {}", f);
    }
    println!();
}

fn test_greedys(foods: &mut [Food], max_units: f64) {
    println!("Use greedy by value to allocate {} calories", max_units);
    let func = |a: &Food, b: &Food| {
        let a_value = a.value();
        let b_value = b.value();
        b_value.partial_cmp(&a_value).unwrap()
    };
    test_greedy(foods, max_units, &func);

    println!("Use greedy by cost to allocate {} calories", max_units);
    let func = |a: &Food, b: &Food| {
        let a_calories = a.calories();
        let b_calories = b.calories();
        a_calories.partial_cmp(&b_calories).unwrap()
    };
    test_greedy(foods, max_units, &func);

    println!("Use greedy by density to allocate {} calories", max_units);
    let func = |a: &Food, b: &Food| {
        let a_density = a.density();
        let b_density = b.density();
        b_density.partial_cmp(&a_density).unwrap()
    };
    test_greedy(foods, max_units, &func);
}

fn greedy(items: &[Food], max_cost: f64) -> (Vec<Food>, f64) {
    let mut results: Vec<Food> = Vec::new();
    let mut total_value: f64 = 0.0;
    let mut total_cost: f64 = 0.0;

    for item in items.iter() {
        let name = item.name();
        let calories = item.calories();
        let value = item.value();
        if (total_cost + calories) <= max_cost {
            results.push(Food::new(name, value, calories));
            total_cost += calories;
            total_value += value;
        }
    }

    (results, total_value)
}
