use anyhow::Result;
use libconcentratord::{gnss, region};
use libloragw_sx1302::hal;
use crate::config::Region;
use crate::config::vendor::{ComType, RadioConfig};
use crate::config;
use super::super::Configuration;
use super::{l1_8XX_A, l1_8XX_B, l1_9XX_A, l1_9XX_B};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelLoraRegion {
    V8XX,
    V9XX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelLoraFrontendRev {
    A,
    B,
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

    let model_lora_frontend_rev = if flags.contains(&"lora_frontend_rev-A".to_string()) {
        ModelLoraFrontendRev::A
    } else if flags.contains(&"lora_frontend_rev-B".to_string()) {
        ModelLoraFrontendRev::B
    } else {
        return Err(anyhow!(
            "You must specify a lora_frontend_rev flag: lora_frontend_rev-A or lora_frontend_rev-B"
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
        full_duplex: false,
        lora_multi_sf_bandwidth: 125000,
        radio_config: vec![
            RadioConfig {
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
                tx_min_max_freqs,
                tx_gain_table: super::get_tx_gain_table(
                    match (model_lora_region, model_lora_frontend_rev) {
                        (ModelLoraRegion::V8XX, ModelLoraFrontendRev::A) => l1_8XX_A::tx_gain_config(),
                        (ModelLoraRegion::V8XX, ModelLoraFrontendRev::B) => l1_8XX_B::tx_gain_config(),
                        (ModelLoraRegion::V9XX, ModelLoraFrontendRev::A) => l1_9XX_A::tx_gain_config(),
                        (ModelLoraRegion::V9XX, ModelLoraFrontendRev::B) => l1_9XX_B::tx_gain_config(),
                    },
                    conf.gateway.antenna_gain,
                ),
            },
            RadioConfig {
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
                tx_min_max_freqs: vec![],
                tx_gain_table: vec![],
            },
        ],
        gps: gnss::Device::None,
        com_type: ComType::Spi,
        com_path: "/dev/spidev0.0".to_string(),
        sx1302_reset_pin: conf.gateway.get_sx1302_reset_pin("/dev/gpiochip0", 1),
        ..Default::default()
    })


}
