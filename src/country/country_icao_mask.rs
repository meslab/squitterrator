pub fn icao_to_country(icao: u32) -> (&'static str, &'static str) {
    let (country, cshrt) = match icao >> 20 {
        0b0001 => ("Russian Federation", "RU"),
        0b1010 => ("United States", "US"),
        _ => match icao >> 18 {
            0b111000 => ("Argentina", "AR"),
            0b011111 => ("Australia", "AU"),
            0b111001 => ("Brazil", "BR"),
            0b110000 => ("Canada", "CA"),
            0b011110 => ("China", "CN"),
            0b001110 => ("France", "FR"),
            0b001111 => ("Germany", "DE"),
            0b100000 => ("India", "IN"),
            0b001100 => ("Italy", "IT"),
            0b100001 => ("Japan", "JP"),
            0b001101 => ("Spain", "ES"),
            0b010000 => ("United Kingdom", "GB"),
            _ => match icao >> 15 {
                0b000010100 => ("Algeria", "DZ"),
                0b010001000 => ("Austria", "AT"),
                0b010001001 => ("Belgium", "BE"),
                0b010001010 => ("Bulgaria", "BG"),
                0b010010011 => ("Czech Republic", "CZ"),
                0b011100100 => ("Democratic People's Republic of Korea", "KP"),
                0b010001011 => ("Denmark", "DK"),
                0b000000010 => ("Egypt", "EG"),
                0b010001100 => ("Finland", "FI"),
                0b010001101 => ("Greece", "GR"),
                0b010001110 => ("Hungary", "HU"),
                0b100010100 => ("Indonesia", "ID"),
                0b011100110 => ("Iran, Islamic Republic of", "IR"),
                0b011100101 => ("Iraq", "IQ"),
                0b011100111 => ("Israel", "IL"),
                0b011101000 => ("Jordan", "JO"),
                0b011101001 => ("Lebanon", "LB"),
                0b000000011 => ("Libyan Arab Jamahiriya", "LY"),
                0b011101010 => ("Malaysia", "MY"),
                0b000011010 => ("Mexico", "MX"),
                0b000000100 => ("Morocco", "MA"),
                0b010010000 => ("Netherlands, Kingdom of the", "NL"),
                0b110010000 => ("New Zealand", "NZ"),
                0b010001111 => ("Norway", "NO"),
                0b011101100 => ("Pakistan", "PK"),
                0b011101011 => ("Philippines", "PH"),
                0b010010001 => ("Poland", "PL"),
                0b010010010 => ("Portugal", "PT"),
                0b011100011 => ("Republic of Korea", "KR"),
                0b010010100 => ("Romania", "RO"),
                0b011100010 => ("Saudi Arabia", "SA"),
                0b011101101 => ("Singapore", "SG"),
                0b000000001 => ("South Africa", "ZA"),
                0b011101110 => ("Sri Lanka", "LK"),
                0b010010101 => ("Sweden", "SE"),
                0b010010110 => ("Switzerland", "CH"),
                0b011101111 => ("Syrian Arab Republic", "SY"),
                0b100010000 => ("Thailand", "TH"),
                0b000000101 => ("Tunisia", "TN"),
                0b010010111 => ("Turkey", "TR"),
                0b010100001 => ("Ukraine", "UA"),
                0b000011011 => ("Venezuela", "VE"),
                0b100010001 => ("Viet Nam", "VN"),
                0b010011000 => ("Yugoslavia", "YU"),
                0b111100000 => ("ICAO1", "ICAO1"),
                _ => match icao >> 12 {
                    0b011100000000 => ("Afghanistan", "AF"),
                    0b000010010000 => ("Angola", "AO"),
                    0b000010101000 => ("Bahamas", "BS"),
                    0b100010010100 => ("Bahrain", "BH"),
                    0b011100000010 => ("Bangladesh", "BD"),
                    0b111010010100 => ("Bolivia", "BO"),
                    0b000010011100 => ("Burkina Faso", "BF"),
                    0b000000110010 => ("Burundi", "BI"),
                    0b011100001110 => ("Cambodia", "KH"),
                    0b000000110100 => ("Cameroon", "CM"),
                    0b000001101100 => ("Central African Republic", "CF"),
                    0b000010000100 => ("Chad", "TD"),
                    0b111010000000 => ("Chile", "CL"),
                    0b000010101100 => ("Colombia", "CO"),
                    0b000000110110 => ("Congo", "CG"),
                    0b000010101110 => ("Costa Rica", "CR"),
                    0b000000111000 => ("Côte d’Ivoire", "CI"),
                    0b000010110000 => ("Cuba", "CU"),
                    0b000010001100 => ("Democratic Republic of the Congo", "CD"),
                    0b000011000100 => ("Dominican Republic", "DO"),
                    0b111010000100 => ("Ecuador", "EC"),
                    0b000010110010 => ("El Salvador", "SV"),
                    0b000001000010 => ("Equatorial Guinea", "GQ"),
                    0b000001000000 => ("Ethiopia", "ET"),
                    0b110010001000 => ("Fiji", "FJ"),
                    0b000000111110 => ("Gabon", "GA"),
                    0b000010011010 => ("Gambia", "GM"),
                    0b000001000100 => ("Ghana", "GH"),
                    0b000010110100 => ("Guatemala", "GT"),
                    0b000001000110 => ("Guinea", "GN"),
                    0b000010110110 => ("Guyana", "GY"),
                    0b000010111000 => ("Haiti", "HT"),
                    0b000010111010 => ("Honduras", "HN"),
                    0b010011001100 => ("Iceland", "IS"),
                    0b010011001010 => ("Ireland", "IE"),
                    0b000010111110 => ("Jamaica", "JM"),
                    0b000001001100 => ("Kenya", "KE"),
                    0b011100000110 => ("Kuwait", "KW"),
                    0b011100001000 => ("Lao People’s Democratic Republic", "LA"),
                    0b000001010000 => ("Liberia", "LR"),
                    0b000001010100 => ("Madagascar", "MG"),
                    0b000001011000 => ("Malawi", "MW"),
                    0b000001011100 => ("Mali", "ML"),
                    0b010011010010 => ("Malta", "MT"),
                    0b000000000110 => ("Mozambique", "MZ"),
                    0b011100000100 => ("Myanmar", "MM"),
                    0b011100001010 => ("Nepal", "NP"),
                    0b000011000000 => ("Nicaragua", "NI"),
                    0b000001100010 => ("Niger", "NE"),
                    0b000001100100 => ("Nigeria", "NG"),
                    0b000011000010 => ("Panama", "PA"),
                    0b100010011000 => ("Papua New Guinea", "PG"),
                    0b111010001000 => ("Paraguay", "PY"),
                    0b111010001100 => ("Peru", "PE"),
                    0b000001101110 => ("Rwanda", "RW"),
                    0b000001110000 => ("Senegal", "SN"),
                    0b000001111000 => ("Somalia", "SO"),
                    0b000001111100 => ("Sudan", "SD"),
                    0b000011001000 => ("Suriname", "SR"),
                    0b000010001000 => ("Togo", "TG"),
                    0b000011000110 => ("Trinidad and Tobago", "TT"),
                    0b000001101000 => ("Uganda", "UG"),
                    0b100010010110 => ("United Arab Emirates", "AE"),
                    0b000010000000 => ("United Republic of Tanzania", "TZ"),
                    0b111010010000 => ("Uruguay", "UY"),
                    0b100010010000 => ("Yemen", "YE"),
                    0b000010001010 => ("Zambia", "ZM"),
                    _ => match icao >> 10 {
                        0b01010000000100 => ("Albania", "AL"),
                        0b00001100101000 => ("Antigua and Barbuda", "AG"),
                        0b01100000000000 => ("Armenia", "AM"),
                        0b01100000000010 => ("Azerbaijan", "AZ"),
                        0b00001010101000 => ("Barbados", "BB"),
                        0b01010001000000 => ("Belarus", "BY"),
                        0b00001010101100 => ("Belize", "BZ"),
                        0b00001001010000 => ("Benin", "BJ"),
                        0b01101000000000 => ("Bhutan", "BT"),
                        0b01010001001100 => ("Bosnia and Herzegovina", "BA"),
                        0b00000011000000 => ("Botswana", "BW"),
                        0b10001001010100 => ("Brunei Darussalam", "BN"),
                        0b00001001011000 => ("Cape Verde", "CV"),
                        0b00000011010100 => ("Comoros", "KM"),
                        0b10010000000100 => ("Cook Islands", "CK"),
                        0b01010000000111 => ("Croatia", "HR"),
                        0b01001100100000 => ("Cyprus", "CY"),
                        0b00001001100000 => ("Djibouti", "DJ"),
                        0b00100000001000 => ("Eritrea", "ER"),
                        0b01010001000100 => ("Estonia", "EE"),
                        0b01010001010000 => ("Georgia", "GE"),
                        0b00001100110000 => ("Grenada", "GD"),
                        0b00000100100000 => ("Guinea-Bissau", "GW"),
                        0b01101000001100 => ("Kazakhstan", "KZ"),
                        0b11001000111000 => ("Kiribati", "KI"),
                        0b01100000000100 => ("Kyrgyzstan", "KG"),
                        0b01010000001011 => ("Latvia", "LV"),
                        0b00000100101000 => ("Lesotho", "LS"),
                        0b01010000001111 => ("Lithuania", "LT"),
                        0b01001101000000 => ("Luxembourg", "LU"),
                        0b00000101101000 => ("Maldives", "MV"),
                        0b10010000000000 => ("Marshall Islands", "MH"),
                        0b00000101111000 => ("Mauritania", "MR"),
                        0b00000110000000 => ("Mauritius", "MU"),
                        0b01101000000100 => ("Micronesia, Federated States of", "FM"),
                        0b01001101010000 => ("Monaco", "MC"),
                        0b01101000001000 => ("Mongolia", "MN"),
                        0b00100000000100 => ("Namibia", "NA"),
                        0b11001000101000 => ("Nauru", "NR"),
                        0b01110000110000 => ("Oman", "OM"),
                        0b01101000010000 => ("Palau", "PW"),
                        0b00000110101000 => ("Qatar", "QA"),
                        0b01010000010011 => ("Republic of Moldova", "MD"),
                        0b11001000110000 => ("Saint Lucia", "LC"),
                        0b00001011110000 => ("Saint Vincent and the Grenadines", "VC"),
                        0b10010000001000 => ("Samoa", "WS"),
                        0b01010000000000 => ("San Marino", "SM"),
                        0b00001001111000 => ("Sao Tome and Principe", "ST"),
                        0b00000111010000 => ("Seychelles", "SC"),
                        0b00000111011000 => ("Sierra Leone", "SL"),
                        0b01010000010111 => ("Slovakia", "SK"),
                        0b01010000011011 => ("Slovenia", "SI"),
                        0b10001001011100 => ("Solomon Islands", "SB"),
                        0b00000111101000 => ("Swaziland", "SZ"),
                        0b01010001010100 => ("Tajikistan", "TJ"),
                        0b01010001001000 => ("The former Yugoslav Republic of Macedonia", "MK"),
                        0b11001000110100 => ("Tonga", "TO"),
                        0b01100000000110 => ("Turkmenistan", "TM"),
                        0b01010000011111 => ("Uzbekistan", "UZ"),
                        0b11001001000000 => ("Vanuatu", "VU"),
                        0b00000000010000 => ("Zimbabwe", "ZW"),
                        0b10001001100100 => ("ICAO2", "ICAO2"),
                        0b11110000100100 => ("ICAO2", "ICAO2"),
                        _ => ("UFO", "??"),
                    },
                },
            },
        },
    };
    (country, cshrt)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_icao_to_country() {
        assert_eq!(
            icao_to_country(0b010010000_000000000000000),
            ("Netherlands, Kingdom of the", "NL")
        );
        assert_eq!(
            icao_to_country(0b110010000_000000000000000),
            ("New Zealand", "NZ")
        );
        assert_eq!(
            icao_to_country(0b010001111_000000000000000),
            ("Norway", "NO")
        );
        assert_eq!(
            icao_to_country(0b011101100_000000000000000),
            ("Pakistan", "PK")
        );
        assert_eq!(
            icao_to_country(0b011101011_000000000000000),
            ("Philippines", "PH")
        );
        assert_eq!(
            icao_to_country(0b010010001_000000000000000),
            ("Poland", "PL")
        );
        assert_eq!(
            icao_to_country(0b010010010_000000000000000),
            ("Portugal", "PT")
        );
        assert_eq!(
            icao_to_country(0b011100011_000000000000000),
            ("Republic of Korea", "KR")
        );
        assert_eq!(
            icao_to_country(0b010010100_000000000000000),
            ("Romania", "RO")
        );
        assert_eq!(
            icao_to_country(0b011100010_000000000000000),
            ("Saudi Arabia", "SA")
        );
        assert_eq!(
            icao_to_country(0b011101101_000000000000000),
            ("Singapore", "SG")
        );
        assert_eq!(
            icao_to_country(0b000000001_000000000000000),
            ("South Africa", "ZA")
        );
        assert_eq!(
            icao_to_country(0b011101110_000000000000000),
            ("Sri Lanka", "LK")
        );
        assert_eq!(
            icao_to_country(0b010010101_000000000000000),
            ("Sweden", "SE")
        );
        assert_eq!(
            icao_to_country(0b010010110_000000000000000),
            ("Switzerland", "CH")
        );
        assert_eq!(
            icao_to_country(0b011101111_000000000000000),
            ("Syrian Arab Republic", "SY")
        );
        assert_eq!(
            icao_to_country(0b100010000_000000000000000),
            ("Thailand", "TH")
        );
        assert_eq!(
            icao_to_country(0b000000101_000000000000000),
            ("Tunisia", "TN")
        );
        assert_eq!(
            icao_to_country(0b010010111_000000000000000),
            ("Turkey", "TR")
        );
        assert_eq!(
            icao_to_country(0b010100001_000000000000000),
            ("Ukraine", "UA")
        );
        assert_eq!(
            icao_to_country(0b000011011_000000000000000),
            ("Venezuela", "VE")
        );
        assert_eq!(
            icao_to_country(0b100010001_000000000000000),
            ("Viet Nam", "VN")
        );
        assert_eq!(
            icao_to_country(0b010011000_000000000000000),
            ("Yugoslavia", "YU")
        );
        assert_eq!(
            icao_to_country(0b111100000_000000000000000),
            ("ICAO1", "ICAO1")
        );
        assert_eq!(
            icao_to_country(0b011100000000_000000000000),
            ("Afghanistan", "AF")
        );
        assert_eq!(
            icao_to_country(0b000010010000_000000000000),
            ("Angola", "AO")
        );
        assert_eq!(
            icao_to_country(0b000010101000_000000000000),
            ("Bahamas", "BS")
        );
        assert_eq!(
            icao_to_country(0b100010010100_000000000000),
            ("Bahrain", "BH")
        );
        assert_eq!(
            icao_to_country(0b011100000010_000000000000),
            ("Bangladesh", "BD")
        );
        assert_eq!(
            icao_to_country(0b111010010100_000000000000),
            ("Bolivia", "BO")
        );
        assert_eq!(
            icao_to_country(0b000010011100_000000000000),
            ("Burkina Faso", "BF")
        );
        assert_eq!(
            icao_to_country(0b000000110010_000000000000),
            ("Burundi", "BI")
        );
        assert_eq!(
            icao_to_country(0b011100001110_000000000000),
            ("Cambodia", "KH")
        );
        assert_eq!(
            icao_to_country(0b000000110100_000000000000),
            ("Cameroon", "CM")
        );
        assert_eq!(
            icao_to_country(0b000001101100_000000000000),
            ("Central African Republic", "CF")
        );
        assert_eq!(icao_to_country(0b000010000100_000000000000), ("Chad", "TD"));
        assert_eq!(
            icao_to_country(0b111010000000_000000000000),
            ("Chile", "CL")
        );
        assert_eq!(
            icao_to_country(0b000010101100_000000000000),
            ("Colombia", "CO")
        );
        assert_eq!(
            icao_to_country(0b000000110110_000000000000),
            ("Congo", "CG")
        );
        assert_eq!(
            icao_to_country(0b000010101110_000000000000),
            ("Costa Rica", "CR")
        );
        assert_eq!(
            icao_to_country(0b000000111000_000000000000),
            ("Côte d’Ivoire", "CI")
        );
        assert_eq!(icao_to_country(0b000010110000_000000000000), ("Cuba", "CU"));
        assert_eq!(
            icao_to_country(0b000010001100_000000000000),
            ("Democratic Republic of the Congo", "CD")
        );
        assert_eq!(
            icao_to_country(0b000011000100_000000000000),
            ("Dominican Republic", "DO")
        );
        assert_eq!(
            icao_to_country(0b111010000100_000000000000),
            ("Ecuador", "EC")
        );
        assert_eq!(
            icao_to_country(0b000010110010_000000000000),
            ("El Salvador", "SV")
        );
        assert_eq!(
            icao_to_country(0b000001000010_000000000000),
            ("Equatorial Guinea", "GQ")
        );
        assert_eq!(
            icao_to_country(0b000001000000_000000000000),
            ("Ethiopia", "ET")
        );
        assert_eq!(icao_to_country(0b110010001000_000000000000), ("Fiji", "FJ"));
        assert_eq!(
            icao_to_country(0b000000111110_000000000000),
            ("Gabon", "GA")
        );
        assert_eq!(
            icao_to_country(0b000010011010_000000000000),
            ("Gambia", "GM")
        );
        assert_eq!(
            icao_to_country(0b000001000100_000000000000),
            ("Ghana", "GH")
        );
        assert_eq!(
            icao_to_country(0b000010110100_000000000000),
            ("Guatemala", "GT")
        );
        assert_eq!(
            icao_to_country(0b000001000110_000000000000),
            ("Guinea", "GN")
        );
        assert_eq!(
            icao_to_country(0b000010110110_000000000000),
            ("Guyana", "GY")
        );
        assert_eq!(
            icao_to_country(0b000010111000_000000000000),
            ("Haiti", "HT")
        );
        assert_eq!(
            icao_to_country(0b000010111010_000000000000),
            ("Honduras", "HN")
        );
        assert_eq!(
            icao_to_country(0b010011001100_000000000000),
            ("Iceland", "IS")
        );
        assert_eq!(
            icao_to_country(0b010011001010_000000000000),
            ("Ireland", "IE")
        );
        assert_eq!(
            icao_to_country(0b000010111110_000000000000),
            ("Jamaica", "JM")
        );
        assert_eq!(
            icao_to_country(0b000001001100_000000000000),
            ("Kenya", "KE")
        );
        assert_eq!(
            icao_to_country(0b011100000110_000000000000),
            ("Kuwait", "KW")
        );
        assert_eq!(
            icao_to_country(0b011100001000_000000000000),
            ("Lao People’s Democratic Republic", "LA")
        );
        assert_eq!(
            icao_to_country(0b000001010000_000000000000),
            ("Liberia", "LR")
        );
        assert_eq!(
            icao_to_country(0b000001010100_000000000000),
            ("Madagascar", "MG")
        );
        assert_eq!(
            icao_to_country(0b000001011000_000000000000),
            ("Malawi", "MW")
        );
        assert_eq!(icao_to_country(0b000001011100_000000000000), ("Mali", "ML"));
        assert_eq!(
            icao_to_country(0b000000000110_000000000000),
            ("Mozambique", "MZ")
        );
        assert_eq!(
            icao_to_country(0b011100000100_000000000000),
            ("Myanmar", "MM")
        );
        assert_eq!(
            icao_to_country(0b011100001010_000000000000),
            ("Nepal", "NP")
        );
        assert_eq!(
            icao_to_country(0b000011000000_000000000000),
            ("Nicaragua", "NI")
        );
        assert_eq!(
            icao_to_country(0b000001100010_000000000000),
            ("Niger", "NE")
        );
        assert_eq!(
            icao_to_country(0b000001100100_000000000000),
            ("Nigeria", "NG")
        );
        assert_eq!(
            icao_to_country(0b000011000010_000000000000),
            ("Panama", "PA")
        );
        assert_eq!(
            icao_to_country(0b100010011000_000000000000),
            ("Papua New Guinea", "PG")
        );
        assert_eq!(
            icao_to_country(0b111010001000_000000000000),
            ("Paraguay", "PY")
        );
        assert_eq!(icao_to_country(0b111010001100_000000000000), ("Peru", "PE"));
        assert_eq!(
            icao_to_country(0b000001101110_000000000000),
            ("Rwanda", "RW")
        );
        assert_eq!(
            icao_to_country(0b000001110000_000000000000),
            ("Senegal", "SN")
        );
        assert_eq!(
            icao_to_country(0b000001111000_000000000000),
            ("Somalia", "SO")
        );
        assert_eq!(
            icao_to_country(0b000001111100_000000000000),
            ("Sudan", "SD")
        );
        assert_eq!(
            icao_to_country(0b000011001000_000000000000),
            ("Suriname", "SR")
        );
        assert_eq!(icao_to_country(0b000010001000_000000000000), ("Togo", "TG"));
        assert_eq!(
            icao_to_country(0b000011000110_000000000000),
            ("Trinidad and Tobago", "TT")
        );
        assert_eq!(
            icao_to_country(0b000001101000_000000000000),
            ("Uganda", "UG")
        );
        assert_eq!(
            icao_to_country(0b100010010110_000000000000),
            ("United Arab Emirates", "AE")
        );
        assert_eq!(
            icao_to_country(0b000010000000_000000000000),
            ("United Republic of Tanzania", "TZ")
        );
        assert_eq!(
            icao_to_country(0b111010010000_000000000000),
            ("Uruguay", "UY")
        );
        assert_eq!(
            icao_to_country(0b100010010000_000000000000),
            ("Yemen", "YE")
        );
        assert_eq!(
            icao_to_country(0b000010001010_000000000000),
            ("Zambia", "ZM")
        );
        assert_eq!(
            icao_to_country(0b01001101001000_0000000000),
            ("Malta", "MT")
        );
        assert_eq!(
            icao_to_country(0b10010000000000_0000000000),
            ("Marshall Islands", "MH")
        );
        assert_eq!(
            icao_to_country(0b00000101111000_0000000000),
            ("Mauritania", "MR")
        );
        assert_eq!(
            icao_to_country(0b00000110000000_0000000000),
            ("Mauritius", "MU")
        );
        assert_eq!(
            icao_to_country(0b01101000000100_0000000000),
            ("Micronesia, Federated States of", "FM")
        );
        assert_eq!(
            icao_to_country(0b01001101010000_0000000000),
            ("Monaco", "MC")
        );
        assert_eq!(
            icao_to_country(0b01101000001000_0000000000),
            ("Mongolia", "MN")
        );
        assert_eq!(
            icao_to_country(0b00100000000100_0000000000),
            ("Namibia", "NA")
        );
        assert_eq!(
            icao_to_country(0b11001000101000_0000000000),
            ("Nauru", "NR")
        );
        assert_eq!(icao_to_country(0b01110000110000_0000000000), ("Oman", "OM"));
        assert_eq!(
            icao_to_country(0b01101000010000_0000000000),
            ("Palau", "PW")
        );
        assert_eq!(
            icao_to_country(0b00000110101000_0000000000),
            ("Qatar", "QA")
        );
        assert_eq!(
            icao_to_country(0b01010000010011_0000000000),
            ("Republic of Moldova", "MD")
        );
        assert_eq!(
            icao_to_country(0b11001000110100_0000000000),
            ("Tonga", "TO")
        );
        assert_eq!(
            icao_to_country(0b01100000000110_0000000000),
            ("Turkmenistan", "TM")
        );
        assert_eq!(
            icao_to_country(0b01010000011111_0000000000),
            ("Uzbekistan", "UZ")
        );
        assert_eq!(
            icao_to_country(0b11001001000000_0000000000),
            ("Vanuatu", "VU")
        );
        assert_eq!(
            icao_to_country(0b00000000010000_0000000000),
            ("Zimbabwe", "ZW")
        );
        assert_eq!(
            icao_to_country(0b10001001100100_0000000000),
            ("ICAO2", "ICAO2")
        );
    }

    #[test]
    fn test_icao_to_country_ufo() {
        assert_eq!(icao_to_country(0b11010000100100_0000000000), ("UFO", "??"));
    }

    #[test]
    fn test_icao_to_country_icao1() {
        assert_eq!(
            icao_to_country(0b11110000000000_0000000000),
            ("ICAO1", "ICAO1")
        );
    }

    #[test]
    fn test_icao_to_country_icao2() {
        assert_eq!(
            icao_to_country(0b10001001100100_0000000000),
            ("ICAO2", "ICAO2")
        );
        assert_eq!(
            icao_to_country(0b11110000100100_0000000000),
            ("ICAO2", "ICAO2")
        );
    }

    #[test]
    fn test_icao_to_country_real() {
        assert_eq!(icao_to_country(0xA8A87E), ("United States", "US"));
        assert_eq!(icao_to_country(0x40643C), ("United Kingdom", "GB"));
        assert_eq!(icao_to_country(0x406B21), ("United Kingdom", "GB"));
        assert_eq!(icao_to_country(0xC004F3), ("Canada", "CA"));
        assert_eq!(icao_to_country(0x4408A2), ("Austria", "AT"));
        assert_eq!(icao_to_country(0x3C56E7), ("Germany", "DE"));
        assert_eq!(icao_to_country(0x471F62), ("Hungary", "HU"));
        assert_eq!(icao_to_country(0x3C4A3D), ("Germany", "DE"));
        assert_eq!(icao_to_country(0x88510A), ("Thailand", "TH"));
        assert_eq!(icao_to_country(0x395D6D), ("France", "FR"));
        assert_eq!(
            icao_to_country(0x485344),
            (("Netherlands, Kingdom of the", "NL"))
        );
        assert_eq!(icao_to_country(0x4CA4A4), ("Ireland", "IE"));
        assert_eq!(icao_to_country(0xE48DF5), ("Brazil", "BR"));
        assert_eq!(icao_to_country(0x502CE5), ("Latvia", "LV"));
        assert_eq!(icao_to_country(0x5100FA), ("Belarus", "BY"));
        assert_eq!(icao_to_country(0x151D83), ("Russian Federation", "RU"));
    }
}
