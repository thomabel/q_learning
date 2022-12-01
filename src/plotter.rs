use plotters::prelude::*;

pub struct QResult {
    pub games_played: usize,
    pub p1_win: usize,
    pub p2_win: usize,
    pub draw: usize,
}
impl QResult {
    pub fn new(games_played: usize) -> QResult {
        QResult { games_played, p1_win: 0, p2_win: 0, draw: 0 }
    }

    pub fn _analyze(&self) {
        // Analyze the results.
        let denom = self.games_played as f32;
        let n = 1000.;
        let p1_ratio = self.p1_win as f32 / denom * n;
        let p2_ratio = self.p2_win as f32 / denom * n;
        let draw_ratio = self.draw as f32 / denom * n;

        print!("Total / P1W / P2W / Draw :: ");
        println!("{:6} / {} ({:3.3}) / {} ({:3.3}) / {} ({:3.3})", 
            self.games_played, 
            self.p1_win, p1_ratio,
            self.p2_win, p2_ratio,
            self.draw, draw_ratio
        );
    }
}

// Uses plotters crate to visualize data.
pub fn visualize(result: Vec<QResult>, file_name: String, title: String)
    -> Result<(), Box<dyn std::error::Error>>
{
    // Basic info
    let path = format!("./plots/{}.png", file_name);
    let size = (1290, 720);
    let root = BitMapBackend::new(&path, size).into_drawing_area();
    let dimension = (0.0..(result.len() as f32), 0.0..(result[0].games_played as f32));
    root.fill(&WHITE)?;

    // Builds the 2D graph.
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(dimension.0, dimension.1)?;

    chart.configure_mesh().draw()?;
 

    let (p1_iter, temp): (Vec<_>, Vec<_>) = result.iter().enumerate()
        .map(|(i, r)| {
            let j = i as f32;
            ((j, r.p1_win as f32), ((j, r.p2_win as f32), (j, r.draw as f32)))
        })
        .unzip();
    let (p2_iter, draw_iter): (Vec<_>, Vec<_>) = temp.into_iter().unzip();

    let point_size = 2;

    // PLAYER 1
    let p1_series = LineSeries::new(p1_iter, BLUE.filled()).point_size(point_size);
    chart.draw_series(p1_series)?
        .label("P1 Win")
        .legend(|(x, y)|
            PathElement::new(vec![(x, y), (x + 20, y)], BLUE)
    );

    // PLAYER 2
    let p2_series = LineSeries::new(p2_iter, RED.filled()).point_size(point_size);
    chart.draw_series(p2_series)?
        .label("P2 Win")
        .legend(|(x, y)|
            PathElement::new(vec![(x, y), (x + 20, y)], RED)
    );

    // DRAWS
    let draws_series = LineSeries::new(draw_iter, GREEN.filled()).point_size(point_size);
    chart.draw_series(draws_series)?
        .label("P2 Win")
        .legend(|(x, y)|
            PathElement::new(vec![(x, y), (x + 20, y)], GREEN)
    );

     // Create the line key.
     chart
     .configure_series_labels()
     .background_style(WHITE.mix(0.8))
     .border_style(BLACK)
     .draw()?;
 
     root.present()?;
     Ok(())
}