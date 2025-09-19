use libloragw_sx1301::hal::TxGainConfig;

pub fn tx_gain_config() -> Vec<TxGainConfig> {
    vec![
        super::gain_param(7, 1, 15, 2),
        super::gain_param(8, 2, 9, 1),
        super::gain_param(9, 2, 12, 3),
        super::gain_param(10, 3, 8, 3),
        super::gain_param(11, 2, 10, 0),
        super::gain_param(12, 3, 9, 3),
        super::gain_param(13, 2, 11, 0),
        super::gain_param(14, 2, 13, 2),
        super::gain_param(15, 3, 8, 1),
        super::gain_param(16, 2, 13, 0),
        super::gain_param(17, 2, 14, 0),
        super::gain_param(18, 3, 10, 2),
        super::gain_param(19, 3, 9, 0),
        super::gain_param(20, 3, 10, 1),
        super::gain_param(21, 3, 13, 3),
        super::gain_param(22, 3, 12, 2),
        super::gain_param(23, 3, 11, 0),
        super::gain_param(24, 3, 13, 2),
        super::gain_param(25, 3, 14, 2),
        super::gain_param(26, 3, 14, 0),
        super::gain_param(27, 3, 15, 0),
    ]
}
