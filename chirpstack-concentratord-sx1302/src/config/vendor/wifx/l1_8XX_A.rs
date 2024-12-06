use libloragw_sx1302::hal;

use super::super::super::super::config;
use super::super::{ComType, Configuration, RadioConfig};

pub fn new(conf: &config::Configuration) -> Configuration {
    Configuration {
        radio_count: 2,
        clock_source: 1,
        full_duplex: false,
        lora_multi_sf_bandwidth: 125000,
        radio_config: vec![
            RadioConfig {
                enable: true,
                radio_type: hal::RadioType::SX1250,
                single_input_mode: true,
                rssi_offset: -215.4,
                rssi_temp_compensation: hal::RssiTempCompensationConfig {
                    coeff_a: 0.0,
                    coeff_b: 0.0,
                    coeff_c: 20.41,
                    coeff_d: 2162.56,
                    coeff_e: 0.0,
                },
                tx_enable: true,
                tx_freq_min: 863000000,
                tx_freq_max: 870000000,
                tx_gain_table: super::get_tx_gain_table(
                    POWER_TABLE_BASE.to_vec(),
                    conf.gateway.antenna_gain,
                ),
            },
            RadioConfig {
                enable: true,
                radio_type: hal::RadioType::SX1250,
                single_input_mode: false,
                rssi_offset: -215.4,
                rssi_temp_compensation: hal::RssiTempCompensationConfig {
                    coeff_a: 0.0,
                    coeff_b: 0.0,
                    coeff_c: 20.41,
                    coeff_d: 2162.56,
                    coeff_e: 0.0,
                },
                tx_enable: false,
                tx_freq_min: 0,
                tx_freq_max: 0,
                tx_gain_table: vec![],
            },
        ],
        gps_tty_path: None,
        com_type: ComType::SPI,
        com_path: "/dev/spidev0.0".to_string(),
        reset_pin: match conf.gateway.reset_pin {
            0 => Some((0, 1)),
            _ => Some((0, conf.gateway.reset_pin)),
        },
        power_en_pin: match conf.gateway.power_en_pin {
            0 => None,
            _ => Some((0, conf.gateway.power_en_pin)),
        },
    }
}

// Power table with an antenna gain of 0dBi
lazy_static! {
    static ref POWER_TABLE_BASE: [hal::TxGainConfig; 21] = [
        gain_param(7, 0, 7),
        gain_param(8, 0, 8),
        gain_param(9, 0, 9),
        gain_param(10, 0, 10),
        gain_param(11, 0, 11),
        gain_param(12, 0, 12),
        gain_param(13, 0, 13),
        gain_param(14, 0, 14),
        gain_param(15, 0, 15),
        gain_param(16, 0, 16),
        gain_param(17, 0, 18),
        gain_param(18, 0, 19),
        gain_param(19, 1, 0),
        gain_param(20, 1, 1),
        gain_param(21, 1, 2),
        gain_param(22, 1, 3),
        gain_param(23, 1, 4),
        gain_param(24, 1, 5),
        gain_param(25, 1, 7),
        gain_param(26, 1, 9),
        gain_param(27, 1, 14),
    ];
}

fn gain_param(rf_power: i8, pa: u8, pwr_idx: u8) -> hal::TxGainConfig {
    hal::TxGainConfig {
        rf_power: rf_power,
        dig_gain: 0,
        pa_gain: pa,
        dac_gain: 0,
        mix_gain: 5,
        offset_i: 0,
        offset_q: 0,
        pwr_idx: pwr_idx,
    }
}
