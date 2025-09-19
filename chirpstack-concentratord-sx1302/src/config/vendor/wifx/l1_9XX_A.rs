use libloragw_sx1302::hal::TxGainConfig;

pub fn tx_gain_config() -> Vec<TxGainConfig> {
    vec![
        super::gain_param(7, 0, 8),
        super::gain_param(8, 0, 9),
        super::gain_param(9, 0, 10),
        super::gain_param(10, 0, 11),
        super::gain_param(11, 0, 12),
        super::gain_param(12, 0, 13),
        super::gain_param(13, 0, 14),
        super::gain_param(14, 0, 15),
        super::gain_param(15, 0, 16),
        super::gain_param(16, 0, 17),
        super::gain_param(17, 0, 18),
        super::gain_param(18, 0, 19),
        super::gain_param(19, 1, 0),
        super::gain_param(20, 0, 22),
        super::gain_param(21, 1, 2),
        super::gain_param(22, 1, 3),
        super::gain_param(23, 1, 4),
        super::gain_param(24, 1, 6),
        super::gain_param(25, 1, 8),
        super::gain_param(26, 1, 11),
        super::gain_param(27, 1, 15),
    ]
}
