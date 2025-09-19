use anyhow::Result;
use libloragw_sx1301::hal;

use super::super::super::super::config::{self, Region};
use super::super::{Configuration, Gps};

pub fn new(conf: &config::Configuration) -> Result<Configuration> {
    let region = conf
        .gateway
        .region
        .ok_or_else(|| anyhow!("You must specify a region"))?;

    let radio_min_max_tx_freq = match region {
        Region::EU868 => vec![(863000000, 870000000), (863000000, 870000000)],
        Region::IN865 => vec![(865000000, 867000000), (865000000, 867000000)],
        Region::RU864 => vec![(863000000, 870000000), (863000000, 870000000)],
        _ => return Err(anyhow!("Region '{region}' is not supported")),
    };

    let enforce_duty_cycle = conf.gateway.model_flags.contains(&"ENFORCE_DC".to_string());

    Ok(Configuration {
        enforce_duty_cycle,
        radio_count: 2,
        clock_source: 1,
        radio_rssi_offset: vec![-166.0, -166.0],
        radio_tx_enabled: vec![true, false],
        radio_type: vec![hal::RadioType::SX1257, hal::RadioType::SX1257],
        radio_min_max_tx_freq,
        radio_tx_notch_freq: vec![0, 0],
        lora_multi_sf_bandwidth: 125000,
        tx_gain_table: get_tx_gain_table(conf.gateway.antenna_gain),
        gps: Gps::None,
        spidev_path: "/dev/spidev0.0".to_string(),
        reset_pin: conf.gateway.get_sx1301_reset_pin("/dev/gpiochip0", 1),
    })
}

// Power table with an antenna gain of 0dBi
lazy_static! {
    static ref POWER_TABLE_BASE: [hal::TxGainConfig; 21] = [
        gain_param(7, 1, 15, 2),
        gain_param(8, 2, 9, 1),
        gain_param(9, 2, 12, 3),
        gain_param(10, 3, 8, 3),
        gain_param(11, 2, 10, 0),
        gain_param(12, 3, 9, 3),
        gain_param(13, 2, 11, 0),
        gain_param(14, 2, 13, 2),
        gain_param(15, 3, 8, 1),
        gain_param(16, 2, 13, 0),
        gain_param(17, 2, 14, 0),
        gain_param(18, 3, 10, 2),
        gain_param(19, 3, 9, 0),
        gain_param(20, 3, 10, 1),
        gain_param(21, 3, 13, 3),
        gain_param(22, 3, 12, 2),
        gain_param(23, 3, 11, 0),
        gain_param(24, 3, 13, 2),
        gain_param(25, 3, 14, 2),
        gain_param(26, 3, 14, 0),
        gain_param(27, 3, 15, 0),
    ];
}

fn gain_param(rf_power: i8, pa: u8, mix: u8, dig: u8) -> hal::TxGainConfig {
    hal::TxGainConfig {
        dig_gain: dig,
        pa_gain: pa,
        dac_gain: 3,
        mix_gain: mix,
        rf_power: rf_power,
    }
}

fn get_tx_gain_table(antenna_gain: i8) -> Vec<hal::TxGainConfig> {
    // filter the power values
    let entries: Vec<hal::TxGainConfig> = POWER_TABLE_BASE
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
