use anyhow::Result;
use libloragw_sx1302::hal;

use super::super::super::super::config::{self, Region};
use super::super::{ComType, Configuration, Gps, RadioConfig};

pub fn new(conf: &config::Configuration) -> Result<Configuration> {
    let region = conf
        .gateway
        .region
        .ok_or_else(|| anyhow!("You must specify a region"))?;

    let (tx_freq_min, tx_freq_max) = match region {
        Region::EU868 => (863000000, 870000000),
        Region::IN865 => (865000000, 867000000),
        Region::RU864 => (863000000, 870000000),
        _ => return Err(anyhow!("Region '{region}' is not supported")),
    };

    let enforce_duty_cycle = conf.gateway.model_flags.contains(&"ENFORCE_DC".to_string());

    Ok(Configuration {
        enforce_duty_cycle,
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
                tx_freq_min,
                tx_freq_max,
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
        gps: Gps::None,
        com_type: ComType::Spi,
        com_path: "/dev/spidev0.0".to_string(),
        i2c_path: None,
        i2c_temp_sensor_addr: None,
        sx1302_reset_pin: conf.gateway.get_sx1302_reset_pin("/dev/gpiochip0", 1),
        sx1302_power_en_pin: None,
        sx1261_reset_pin: None,
        ad5338r_reset_pin: None,
        reset_commands: None,
    })
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
