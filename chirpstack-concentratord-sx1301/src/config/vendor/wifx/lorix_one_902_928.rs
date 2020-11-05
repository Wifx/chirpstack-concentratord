use libloragw_sx1301::hal;

use super::super::super::super::config;
use super::super::Configuration;

pub fn new(conf: &config::Configuration) -> Configuration {
    Configuration {
        radio_count: 2,
        clock_source: 1,
        radio_rssi_offset: vec![-164.0, -164.0],
        radio_tx_enabled: vec![true, false],
        radio_type: vec![hal::RadioType::SX1257, hal::RadioType::SX1257],
        radio_min_max_tx_freq: vec![(863000000, 870000000), (863000000, 870000000)],
        radio_tx_notch_freq: vec![129000, 0],
        lora_multi_sf_bandwidth: 125000,
        tx_gain_table: match conf.gateway.antenna_gain {
            2 => vec![
                hal::TxGainConfig { rf_power: -6, pa_gain: 0, mix_gain:  9, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: -3, pa_gain: 0, mix_gain: 14, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  0, pa_gain: 0, mix_gain: 15, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  3, pa_gain: 2, mix_gain:  8, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  6, pa_gain: 1, mix_gain: 11, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 10, pa_gain: 2, mix_gain:  9, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 11, pa_gain: 2, mix_gain: 12, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 12, pa_gain: 3, mix_gain:  8, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 13, pa_gain: 2, mix_gain: 10, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 14, pa_gain: 2, mix_gain: 12, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 16, pa_gain: 2, mix_gain: 13, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 20, pa_gain: 3, mix_gain: 10, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 23, pa_gain: 3, mix_gain: 13, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 25, pa_gain: 3, mix_gain: 11, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 26, pa_gain: 3, mix_gain: 12, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 27, pa_gain: 3, mix_gain: 13, dig_gain: 0, dac_gain: 3 }
            ],
            3 => vec![
                hal::TxGainConfig { rf_power: -6, pa_gain: 0, mix_gain:  9, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: -3, pa_gain: 0, mix_gain: 10, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  0, pa_gain: 0, mix_gain: 13, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  3, pa_gain: 1, mix_gain: 11, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  6, pa_gain: 1, mix_gain: 10, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 10, pa_gain: 1, mix_gain: 14, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 11, pa_gain: 1, mix_gain: 14, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 12, pa_gain: 1, mix_gain: 15, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 13, pa_gain: 2, mix_gain: 10, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 14, pa_gain: 2, mix_gain: 10, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 16, pa_gain: 2, mix_gain: 11, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 20, pa_gain: 2, mix_gain: 14, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 23, pa_gain: 3, mix_gain: 11, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 25, pa_gain: 3, mix_gain: 12, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 26, pa_gain: 3, mix_gain: 11, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 27, pa_gain: 3, mix_gain: 13, dig_gain: 2, dac_gain: 3 }
            ],
            4 => vec![
                hal::TxGainConfig { rf_power: -6, pa_gain: 0, mix_gain:  9, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: -3, pa_gain: 0, mix_gain: 13, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  0, pa_gain: 0, mix_gain: 13, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  3, pa_gain: 1, mix_gain:  9, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  6, pa_gain: 1, mix_gain: 10, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 10, pa_gain: 1, mix_gain: 14, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 11, pa_gain: 1, mix_gain: 14, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 12, pa_gain: 2, mix_gain: 12, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 13, pa_gain: 3, mix_gain:  8, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 14, pa_gain: 2, mix_gain: 13, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 16, pa_gain: 2, mix_gain: 11, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 20, pa_gain: 2, mix_gain: 14, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 23, pa_gain: 3, mix_gain: 10, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 25, pa_gain: 3, mix_gain: 11, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 26, pa_gain: 3, mix_gain: 11, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 27, pa_gain: 3, mix_gain: 13, dig_gain: 2, dac_gain: 3 }
            ],
            5 => vec![
                hal::TxGainConfig { rf_power: -6, pa_gain: 0, mix_gain:  8, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: -3, pa_gain: 0, mix_gain:  9, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  0, pa_gain: 0, mix_gain: 12, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  3, pa_gain: 0, mix_gain: 14, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power:  6, pa_gain: 2, mix_gain:  8, dig_gain: 3, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 10, pa_gain: 1, mix_gain: 11, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 11, pa_gain: 1, mix_gain: 12, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 12, pa_gain: 2, mix_gain:  8, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 13, pa_gain: 1, mix_gain: 14, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 14, pa_gain: 1, mix_gain: 15, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 16, pa_gain: 2, mix_gain: 10, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 20, pa_gain: 2, mix_gain: 13, dig_gain: 1, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 23, pa_gain: 3, mix_gain: 10, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 25, pa_gain: 3, mix_gain: 11, dig_gain: 2, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 26, pa_gain: 3, mix_gain: 10, dig_gain: 0, dac_gain: 3 },
                hal::TxGainConfig { rf_power: 27, pa_gain: 3, mix_gain: 12, dig_gain: 2, dac_gain: 3 }
            ],
            _ => panic!("Invalid antenna_gain: {}", conf.gateway.antenna_gain)
        },
        gps_tty_path: None,
        spidev_path: "/dev/spidev0.0".to_string(),
        reset_pin: Some(1),
    }
}
