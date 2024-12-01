use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use csv::Reader;
use plotters::prelude::*;

#[derive(Debug, Deserialize)]
pub struct GameState {
    time: f64,
    team: Option<u32>,
    player_name: String,
    location_x: f64,
    location_y: f64,
    location_z: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
    rotation_w: f64,
    angular_velocity_x: f64,
    angular_velocity_y: f64,
    angular_velocity_z: f64,
    linear_velocity_x: f64,
    linear_velocity_y: f64,
    linear_velocity_z: f64,
}

pub fn plot_csv(file_path: &str) -> Result<Vec<GameState>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in reader.deserialize() {
        let record: GameState = result?;
        data.push(record);
    }
    let image_name = format!("{}.png", file_path);
    plot_match(&data,&image_name)?;
    println!("Plot saved to {}.png", file_path);
    Ok(data)

}

fn plot_match(data: &[GameState], output_file: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Match Visualization", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-300000.0..300000.0, -500000.0..500000.0)?;

    chart.configure_mesh().draw()?;

    // Plot players and the ball
    chart
        .draw_series(data.iter().filter(|s| s.player_name == "_ball_").map(|state| {
            Circle::new((state.location_x, state.location_y), 5, RED.filled())
        }))?
        .label("Ball")
        .legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));

    chart
        .draw_series(data.iter().filter(|s| s.team == Some(46)).map(|state| {
            Circle::new((state.location_x, state.location_y), 5, BLUE.filled())
        }))?
        .label("Team 46")
        .legend(|(x, y)| Circle::new((x, y), 5, BLUE.filled()));

    chart
        .draw_series(data.iter().filter(|s| s.team == Some(50)).map(|state| {
            Circle::new((state.location_x, state.location_y), 5, GREEN.filled())
        }))?
        .label("Team 50")
        .legend(|(x, y)| Circle::new((x, y), 5, GREEN.filled()));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}


