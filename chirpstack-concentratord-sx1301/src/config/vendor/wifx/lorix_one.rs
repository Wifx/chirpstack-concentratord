use super::super::Configuration;
use super::{lorix_one_8XX, lorix_one_9XX};
use crate::config;
use crate::config::Region;
use anyhow::Result;
use libconcentratord::{gnss, region};
use libloragw_sx1301::hal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelLoraRegion {
    V8XX,
    V9XX,
}

pub fn new(conf: &config::Configuration) -> Result<Configuration> {
    let flags = &conf.gateway.model_flags;

    let model_lora_region = if flags.contains(&"lora_region-8XX".to_string()) {
        ModelLoraRegion::V8XX
    } else if flags.contains(&"lora_region-9XX".to_string()) {
        ModelLoraRegion::V9XX
    } else {
        return Err(anyhow!(
            "You must specify a lora_region flag: lora_region-8XX or lora_region-9XX"
        ));
    };

    let region = conf
        .gateway
        .region
        .ok_or_else(|| anyhow!("You must specify a region"))?;

    let tx_min_max_freqs = match model_lora_region {
        ModelLoraRegion::V8XX => match region {
            Region::EU868 => region::eu868::TX_MIN_MAX_FREQS.to_vec(),
            Region::IN865 => region::in865::TX_MIN_MAX_FREQS.to_vec(),
            Region::RU864 => region::ru864::TX_MIN_MAX_FREQS.to_vec(),
            _ => return Err(anyhow!("Region is not supported: {}", region)),
        },
        ModelLoraRegion::V9XX => match region {
            Region::AS923 | Region::AS923_2 | Region::AS923_3 | Region::AS923_4 => {
                region::as923::TX_MIN_MAX_FREQS.to_vec()
            }
            Region::AU915 => region::au915::TX_MIN_MAX_FREQS.to_vec(),
            Region::US915 => region::us915::TX_MIN_MAX_FREQS.to_vec(),
            _ => return Err(anyhow!("Region is not supported: {}", region)),
        },
    };

    let enforce_duty_cycle = conf.gateway.model_flags.contains(&"ENFORCE_DC".to_string());

    Ok(Configuration {
        enforce_duty_cycle,
        radio_count: 2,
        clock_source: 1,
        radio_rssi_offset: vec![-166.0, -166.0],
        radio_tx_enabled: vec![true, false],
        radio_type: vec![hal::RadioType::SX1257, hal::RadioType::SX1257],
        tx_min_max_freqs,
        radio_tx_notch_freq: vec![0, 0],
        lora_multi_sf_bandwidth: 125000,
        tx_gain_table: super::get_tx_gain_table(
            match model_lora_region {
                (ModelLoraRegion::V8XX) => lorix_one_8XX::tx_gain_config(),
                (ModelLoraRegion::V9XX) => lorix_one_9XX::tx_gain_config(),
            },
            conf.gateway.antenna_gain,
        ),
        gps: gnss::Device::None,
        spidev_path: "/dev/spidev0.0".to_string(),
        reset_pin: conf.gateway.get_sx1301_reset_pin("/dev/gpiochip0", 1),
    })
}
