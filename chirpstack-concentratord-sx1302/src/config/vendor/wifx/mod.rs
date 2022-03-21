pub mod l1_863_870;
pub mod l1_902_928;

use libloragw_sx1302::hal;

fn get_tx_gain_table(
    base_table: Vec<hal::TxGainConfig>,
    antenna_gain: i8,
) -> Vec<hal::TxGainConfig> {
    // filter the power values
    let entries: Vec<hal::TxGainConfig> = base_table
        .iter()
        .map(|entry| entry.clone())
        .filter(|entry| {
            let radiated_power = entry.rf_power + antenna_gain;
            if radiated_power > 27 {
                debug!(
                    "Power table entry including antenna gain is too high, radiated power: {}, discarded",
                    radiated_power
                );
                    return false;
                }
            return true;
        })
        .collect();

    let count = entries.len();
    if count == 0 {
        panic!("No entries available for power table");
    }
    let mut start_index = 0;
    if count > 16 {
        start_index = count - 16;
    }

    // keep only max 16 higher entries
    entries.as_slice()[start_index..].to_vec()
}
