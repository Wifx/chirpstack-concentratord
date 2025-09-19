use libloragw_sx1301::hal::TxGainConfig;

pub fn tx_gain_config() -> Vec<TxGainConfig> {
    vec![
        super::gain_param(7, 1, 12, 2),
        super::gain_param(8, 1, 12, 1),
        super::gain_param(9, 1, 12, 0),
        super::gain_param(10, 1, 13, 0),
        super::gain_param(11, 1, 15, 1),
        super::gain_param(12, 2, 9, 0),
        super::gain_param(13, 3, 8, 3),
        super::gain_param(14, 2, 10, 0),
        super::gain_param(15, 3, 9, 3),
        super::gain_param(16, 2, 12, 1),
        super::gain_param(17, 3, 10, 3),
        super::gain_param(18, 2, 14, 2),
        super::gain_param(19, 2, 15, 2),
        super::gain_param(20, 3, 9, 1),
        super::gain_param(21, 3, 12, 3),
        super::gain_param(22, 3, 10, 1),
        super::gain_param(23, 3, 11, 2),
        super::gain_param(24, 3, 11, 1),
        super::gain_param(25, 3, 14, 3),
        super::gain_param(26, 3, 13, 2),
        super::gain_param(27, 3, 13, 1),
    ]
}
