use std::time;

use errors::*;
use {OutputParameters, Parameters};

/// Handles the `analyse` subcommand.
pub fn handle(
    mut params: Parameters,
    optimize: bool,
    output_params: OutputParameters,
) -> CliResult<()> {
    let model = &mut params.tfd_model;

    info!("Running analyse");
    let start = time::Instant::now();
    let analyse_result = model.analyse();
    let elapsed = start.elapsed();

    let elapsed = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;
    info!("Ran analyse in {:?}ms", elapsed);

    if analyse_result.is_ok() && optimize {
        info!(
            "Size of the graph before pruning: {:?} nodes.",
            model.nodes().len()
        );

        unimplemented!();
        /*
        let model = analyser.to_optimized_model()?;
        analyser = model.analyser()?.with_input_hints(params.inputs)?;

        info!("Running analyse on optimized graph");
        let start = time::Instant::now();
        analyser.analyse()?;
        let elapsed = start.elapsed();
        let elapsed = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;
        info!("Ran second analyse in {:?}ms", elapsed);

        info!(
            "Size of the graph after pruning: approx. {:?} nodes.",
            model.nodes().len()
        );
        */
    }

    let display = ::display_graph::DisplayGraph::from_model(model)?
        .with_graph_def(&params.graph)?;
    display.render(&output_params)?;

    Ok(analyse_result?)
}