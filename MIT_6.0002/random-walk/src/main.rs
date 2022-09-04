mod drunk;
mod field;
mod location;

mod prelude {
    pub use crate::drunk::*;
    pub use crate::field::*;
    pub use crate::location::*;
}

use prelude::*;

use plotters::prelude::*;

fn main() {
    test_sanity();

    println!("drunk test");
    test_walk();

    println!("plot mean value of different drunk");
    test_plot_all();

    println!("plot location");
    test_plot_locs();

    println!("trace walk");
    let mut fields: Vec<Field> = Vec::new();
    let field = Field::new();
    fields.push(field);
    let mut field = Field::new();
    field.set_worm_holes(1000, 100, 100);
    field.set_name("Odd Field".to_string());
    fields.push(field);
    trace_walk(&mut fields, 500, 100.0, 100.0)
}

fn walk(f: &mut Field, d: &Drunk, num_steps: u32) -> f64 {
    let start = f.get_location(d);
    for _ in 0..num_steps {
        f.move_drunk(d);
    }
    let loc = f.get_location(d);
    start.distance_from(&loc)
}

fn test_sanity() {
    let steps = vec![
        Location::new(0.0, 1.0),
        Location::new(0.0, -1.0),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let usual_drunk = Drunk::new("usual".to_owned(), &steps);
    println!("{}", usual_drunk);

    let steps = vec![
        Location::new(0.0, 1.1),
        Location::new(0.0, -0.9),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let masochist_drunk = Drunk::new("masochist".to_owned(), &steps);
    println!("{}", masochist_drunk);

    let mut f = Field::new();
    let origin = Location::new(0.0, 0.0);
    println!("New Field {:?}", f);
    f.add_drunk(&usual_drunk, &origin);
    println!("add usual {:?}", f);
    f.add_drunk(&masochist_drunk, &origin);
    println!("add masochist {:?}", f);

    let dist = walk(&mut f, &usual_drunk, 10000);
    println!("distance = {}", dist);
    let dist = walk(&mut f, &masochist_drunk, 10000);
    println!("distance = {}", dist);
}

fn sim_walks(num_steps: u32, num_trials: u32, drunk: &Drunk) -> Vec<f64> {
    let origin = Location::new(0.0, 0.0);
    let mut distances: Vec<f64> = Vec::new();
    for _ in 0..num_trials {
        let mut f = Field::new();
        f.add_drunk(drunk, &origin);
        distances.push(walk(&mut f, drunk, num_steps));
    }
    distances
}

fn drunck_test(walk_lengths: &[u32], num_trials: u32, drunk: &Drunk) {
    for num_steps in walk_lengths {
        let distances = sim_walks(*num_steps, num_trials, drunk);
        println!("random walk of {} steps", num_steps);
        let sum: f64 = distances.iter().sum();
        let mut min = *distances.first().unwrap();
        let mut max = *distances.first().unwrap();
        for d in distances {
            if d > max {
                max = d;
            }
            if d < min {
                min = d;
            }
        }
        println!(" Mean = {}", sum / (num_trials as f64));
        println!(" Min = {}, Max = {}", min, max);
    }
}

fn test_walk() {
    let steps = vec![
        Location::new(0.0, 1.0),
        Location::new(0.0, -1.0),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let usual_drunk = Drunk::new("usual".to_owned(), &steps);

    let steps = vec![
        Location::new(0.0, 1.1),
        Location::new(0.0, -0.9),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let masochist_drunk = Drunk::new("masochist".to_owned(), &steps);

    let test_steps = vec![1000, 10000];
    println!("usual drunk test");
    drunck_test(&test_steps, 100, &usual_drunk);
    println!("masochist drunk test");
    drunck_test(&test_steps, 100, &masochist_drunk);
}

fn sim_drunk(num_trials: u32, drunk: &Drunk, walk_lengths: &[u32]) -> Vec<f64> {
    let mut mean_distances: Vec<f64> = Vec::new();
    for num_steps in walk_lengths {
        println!("Start simulation of {num_steps} steps");
        let trials = sim_walks(*num_steps, num_trials, drunk);
        let sum: f64 = trials.iter().sum();
        let mean = sum / trials.len() as f64;
        mean_distances.push(mean);
    }
    mean_distances
}

fn sim_all(
    drunks: &[Drunk],
    walk_lengths: &[u32],
    num_trials: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("points.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin::<u32, u32, u32, u32>(20, 20, 20, 20);
    let title = format!("Mean Distance from Origin {num_trials} trials");

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24).into_font())
        .x_label_area_size::<u32>(20)
        .y_label_area_size::<u32>(40)
        .build_cartesian_2d(0f32..100_000f32, 0f32..6_000f32)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.0}", x))
        .x_label_formatter(&|x| format!("{:.0}", x))
        .y_desc("Distance from Origin")
        .x_desc("Number of Steps")
        .axis_desc_style(("sans-serif", 18))
        .draw()?;

    for (i, drunk) in drunks.iter().enumerate() {
        println!("Start simulation of {}", drunk.name());
        let means = sim_drunk(num_trials, drunk, walk_lengths);
        println!("means = {:?}", means);
        let mut points: Vec<(f32, f32)> = Vec::new();
        for i in 0..walk_lengths.len() {
            let x = walk_lengths[i] as f32;
            let y = means[i] as f32;
            points.push((x, y));
        }
        for point in points.iter() {
            println!("{:?}", point);
        }
        let color = if i == 0 { RED } else { GREEN };
        chart.draw_series(LineSeries::new(points.clone(), &color))?;

        if i == 0 {
            chart
                .draw_series(points.iter().map(|point| Circle::new(*point, 5, &RED)))?
                .label(drunk.name())
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        } else {
            chart
                .draw_series(
                    points
                        .iter()
                        .map(|point| TriangleMarker::new(*point, 5, &GREEN)),
                )?
                .label(drunk.name())
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
        };
    }
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .label_font(("sans-serif", 18))
        .draw()?;

    Ok(())
}

fn test_plot_all() {
    let steps = vec![
        Location::new(0.0, 1.0),
        Location::new(0.0, -1.0),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let usual_drunk = Drunk::new("usual".to_owned(), &steps);

    let steps = vec![
        Location::new(0.0, 1.1),
        Location::new(0.0, -0.9),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let masochist_drunk = Drunk::new("masochist".to_owned(), &steps);

    let drunks = vec![usual_drunk, masochist_drunk];
    let num_steps = vec![10, 100, 1000, 10_000, 100_000];
    sim_all(&drunks, &num_steps, 100).unwrap();
}

fn get_final_locs(num_steps: u32, num_trials: u32, drunk: &Drunk) -> Vec<Location> {
    let mut locs: Vec<Location> = Vec::new();
    for _ in 0..num_trials {
        let mut f = Field::new();
        let origin = Location::new(0.0, 0.0);
        f.add_drunk(drunk, &origin);
        for _ in 0..num_steps {
            f.move_drunk(drunk);
        }
        let loc = f.get_location(drunk);
        locs.push(loc);
    }
    locs
}

fn plot_locs(drunks: &[Drunk], num_steps: u32, num_trials: u32) {
    let root_area = BitMapBackend::new("scatter.png", (1024, 1024)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let title = format!("Location at End of Walks ({} steps)", num_steps);
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size::<u32>(LabelAreaPosition::Left, 40)
        .set_label_area_size::<u32>(LabelAreaPosition::Bottom, 40)
        .caption(title, ("sans-serif", 24))
        .build_cartesian_2d(-1000.0..1000.0, -1000.0..1000.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    for (i, drunk) in drunks.iter().enumerate() {
        let locs = get_final_locs(num_steps, num_trials, drunk);
        let sum_x: f64 = locs.iter().map(|l| l.x()).sum();
        let sum_y: f64 = locs.iter().map(|l| l.y()).sum();
        let mean_x = sum_x / locs.len() as f64;
        let mean_y = sum_y / locs.len() as f64;
        let legend_text = format!("{} mean abs dist = <{}, {}>", drunk.name(), mean_x, mean_y);

        let mut points: Vec<(f64, f64)> = Vec::new();
        for loc in locs {
            points.push((loc.x(), loc.y()));
        }

        ctx.draw_series(
            points
                .iter()
                .map(|point| Circle::new(*point, 5, &Palette99::pick(i))),
        )
        .unwrap()
        .label(legend_text)
        .legend(move |(x, y)| Circle::new((x + 10, y), 5, &Palette99::pick(i)));
    }

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .label_font(("sans-serif", 18))
        .draw()
        .unwrap();
}

fn test_plot_locs() {
    let steps = vec![
        Location::new(0.0, 1.0),
        Location::new(0.0, -1.0),
        Location::new(1.0, 0.0),
        Location::new(-1.0, 0.0),
    ];
    let usual_drunk = Drunk::new("usual".to_owned(), &steps);

    let steps = vec![
        Location::new(0.0, 1.1),
        Location::new(0.0, -0.9),
        Location::new(1.1, 0.0),
        Location::new(-0.9, 0.0),
    ];
    let masochist_drunk = Drunk::new("masochist".to_owned(), &steps);

    let drunks = vec![usual_drunk, masochist_drunk];
    plot_locs(&drunks, 10_000, 1_000);
}

fn trace_walk(fields: &mut [Field], num_steps: u32, x_range: f64, y_range: f64) {
    let root_area = BitMapBackend::new("trace_walk.png", (1024, 1024)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let title = format!("Spots Visited on Walk {} steps", num_steps);
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size::<u32>(LabelAreaPosition::Left, 40)
        .set_label_area_size::<u32>(LabelAreaPosition::Bottom, 40)
        .caption(title, ("sans-serif", 24))
        .build_cartesian_2d(-x_range..x_range, -y_range..y_range)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    for (i, field) in fields.iter().enumerate() {
        let mut field = field.clone();
        let steps = vec![
            Location::new(0.0, 1.0),
            Location::new(0.0, -1.0),
            Location::new(1.0, 0.0),
            Location::new(-1.0, 0.0),
        ];
        let usual_drunk = Drunk::new("usual".to_owned(), &steps);
        let origin = Location::new(0.0, 0.0);
        field.add_drunk(&usual_drunk, &origin);
        let mut points: Vec<(f64, f64)> = Vec::new();
        for _ in 0..num_steps {
            field.move_drunk(&usual_drunk);
            let loc = field.get_location(&usual_drunk);
            points.push((loc.x(), loc.y()));
        }

        if i == 0 {
            ctx.draw_series(points.iter().map(|point| Circle::new(*point, 5, &RED)))
                .unwrap()
                .label(field.name())
                .legend(|(x, y)| Circle::new((x + 10, y), 5, &RED));
        } else {
            ctx.draw_series(points.iter().map(|point| Cross::new(*point, 5, &GREEN)))
                .unwrap()
                .label(field.name())
                .legend(|(x, y)| Cross::new((x + 10, y), 5, &GREEN));
        }
    }

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .label_font(("sans-serif", 18))
        .draw()
        .unwrap();
}
