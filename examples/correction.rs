use pzip::correction::{Context, Correction, CorrectionContextTrait};


#[allow(unused_variables, unused_imports)]
fn main() {
    use log::{debug, error, info, trace, warn};
    use pzip::testing::{FileToBeCompressed, Source};
    use std::env::args;

    // Setup of environment
    env_logger::init();
    let arguments: Vec<_> = args().collect();
    let filename = &arguments[1];
    let mut source: Source<f32> = Source::new(filename);
    source.load().unwrap();

    // Subsetting of data
    let data: Vec<u32> = source.data.iter().map(|x| x.to_bits()).collect();
    let (start, size) = (30356, 1000);
    let data = data[start..start + size].to_vec(); // let data: Vec<u32> = vec![4,5,6,8,10,9,0];

    // Get results of uncorrected last value prediction
    let mut uncorrected_last_value_prediction = vec![0u32; data.len()];
    for v in 1..data.len() {
        uncorrected_last_value_prediction[v] = data[v - 1];
    }

    // Get results of corrected last value prediction
    let mut corrected_last_value_prediction: Vec<u32> = Vec::new();
    let mut pred = 0u32;
    let mut ctx = Context::new(1, 3);

    // Decision about method
    let mut method = Correction::DeltaToPowerOf2;

    // Calculate correction and iterate the vector
    info!(",index,uncorrected,pred,truth,m_overshot,m_offset,_");
    for (i, value) in data.iter().enumerate() {
        let uncorrected = pred;
        pred = method.apply_correction(&pred, &mut ctx);
        debug!("IX: {:08} Uncorrected: {:032b}", i, uncorrected);
        debug!("IX: {:08}   Corrected: {:032b} by {:?}", i, pred, ctx);
        debug!("IX: {:08}       Truth: {:032b}", i, value);
        let (before, after) = (
            (uncorrected ^ value).leading_zeros(),
            (pred ^ value).leading_zeros(),
        );
        if before > after {
            warn!("              Degradation: {:02}", before - after);
        } else if before == after {
            debug!("               No change: {:02}", after);
        } else {
            debug!("             Improvement: {:02}", after - before);
        }
        info!(
            ",{},{},{},{},{},{},{}",
            i, uncorrected, pred, value, ctx.overshot, ctx.offset, ""
        );
        corrected_last_value_prediction.push(pred);
        ctx.prediction = pred;
        ctx.truth = *value;
        method.update(&mut ctx);
        pred = *value;
    }

    // Analysis
    let lzc_data: u32 = data.iter().map(|x| x.leading_zeros()).sum();
    let lzc_uncorrected_pred: Vec<u32> = data
        .iter()
        .zip(uncorrected_last_value_prediction.iter())
        .map(|(t, p)| (t ^ p).leading_zeros())
        .collect();
    let lzc_uncorrected_pred_sum: u32 = lzc_uncorrected_pred.iter().sum();
    let lzc_corrected_pred: Vec<u32> = data
        .iter()
        .zip(corrected_last_value_prediction.iter())
        .map(|(t, p)| (t ^ p).leading_zeros())
        .collect();
    let lzc_corrected_pred_sum: u32 = lzc_corrected_pred.iter().sum();

    let same: u32 = lzc_corrected_pred
        .iter()
        .zip(lzc_uncorrected_pred.iter())
        .filter(|(c, u)| c == u)
        .fold(0, |sum, _| sum + 1);
    let better: u32 = lzc_corrected_pred
        .iter()
        .zip(lzc_uncorrected_pred.iter())
        .filter(|(c, u)| c > u)
        .fold(0, |sum, _| sum + 1);
    let worse = data.len() as u32 - same - better;

    let sum_bits = data.len() * 32;
    info!("METD: {:?}", ctx);
    info!("FILE: {}", filename);
    info!("====");
    info!("TOTL: {}", sum_bits);
    info!(
        "ORIG: {} ({:.4}%)",
        lzc_data,
        lzc_data as f32 / sum_bits as f32 * 100.0
    );
    info!(
        "UNCO: {} ({:.4}%)",
        lzc_uncorrected_pred_sum,
        lzc_uncorrected_pred_sum as f32 / sum_bits as f32 * 100.0
    );
    info!(
        "CORR: {} ({:.4}%)",
        lzc_corrected_pred_sum,
        lzc_corrected_pred_sum as f32 / sum_bits as f32 * 100.0
    );
    info!("====");
    info!(
        "SAME: {} ({:.4}%)",
        same,
        same as f32 / data.len() as f32 * 100.0
    );
    info!(
        "BETT: {} ({:.4}%)",
        better,
        better as f32 / data.len() as f32 * 100.0
    );
    info!(
        "WORS: {} ({:.4}%)",
        worse,
        worse as f32 / data.len() as f32 * 100.0
    );
}
