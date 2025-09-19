use libloragw_sx1302::hal::TxGainConfig;

pub fn tx_gain_config() -> Vec<TxGainConfig> {
    vec![
        super::gain_param(7, 0, 7),
        super::gain_param(8, 0, 8),
        super::gain_param(9, 0, 9),
        super::gain_param(10, 0, 10),
        super::gain_param(11, 0, 11),
        super::gain_param(12, 0, 12),
        super::gain_param(13, 0, 13),
        super::gain_param(14, 0, 14),
        super::gain_param(15, 0, 15),
        super::gain_param(16, 0, 16),
        super::gain_param(17, 0, 17),
        super::gain_param(18, 1, 2),
        super::gain_param(19, 0, 18),
        super::gain_param(20, 0, 19),
        super::gain_param(21, 1, 6),
        super::gain_param(22, 1, 7),
        super::gain_param(23, 1, 8),
        super::gain_param(24, 1, 9),
        super::gain_param(25, 1, 10),
        super::gain_param(26, 1, 12),
        super::gain_param(27, 1, 14),
    ]
}
