use super::wallet::Wallet;
use num_bigint::BigInt;
use std::collections::BTreeMap;
use std::str::FromStr;

pub fn wallets() -> BTreeMap<u8, Wallet> {
    let get_bigint = |n| BigInt::from_str(n).expect("Erro obter BigInt");

    BTreeMap::from([
        (
            1,
            Wallet {
                min: get_bigint("1"),
                max: get_bigint("1"),
                address: "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH".to_string(),
                status: 1,
            },
        ),
        (
            2,
            Wallet {
                min: get_bigint("2"),
                max: get_bigint("3"),
                address: "1CUNEBjYrCn2y1SdiUMohaKUi4wpP326Lb".to_string(),
                status: 1,
            },
        ),
        (
            3,
            Wallet {
                min: get_bigint("4"),
                max: get_bigint("7"),
                address: "19ZewH8Kk1PDbSNdJ97FP4EiCjTRaZMZQA".to_string(),
                status: 1,
            },
        ),
        (
            4,
            Wallet {
                min: get_bigint("8"),
                max: get_bigint("15"),
                address: "1EhqbyUMvvs7BfL8goY6qcPbD6YKfPqb7e".to_string(),
                status: 1,
            },
        ),
        (
            5,
            Wallet {
                min: get_bigint("16"),
                max: get_bigint("31"),
                address: "1E6NuFjCi27W5zoXg8TRdcSRq84zJeBW3k".to_string(),
                status: 1,
            },
        ),
        (
            6,
            Wallet {
                min: get_bigint("32"),
                max: get_bigint("63"),
                address: "1PitScNLyp2HCygzadCh7FveTnfmpPbfp8".to_string(),
                status: 1,
            },
        ),
        (
            7,
            Wallet {
                min: get_bigint("64"),
                max: get_bigint("127"),
                address: "1McVt1vMtCC7yn5b9wgX1833yCcLXzueeC".to_string(),
                status: 1,
            },
        ),
        (
            8,
            Wallet {
                min: get_bigint("128"),
                max: get_bigint("255"),
                address: "1M92tSqNmQLYw33fuBvjmeadirh1ysMBxK".to_string(),
                status: 1,
            },
        ),
        (
            9,
            Wallet {
                min: get_bigint("256"),
                max: get_bigint("511"),
                address: "1CQFwcjw1dwhtkVWBttNLDtqL7ivBonGPV".to_string(),
                status: 1,
            },
        ),
        (
            10,
            Wallet {
                min: get_bigint("512"),
                max: get_bigint("1023"),
                address: "1LeBZP5QCwwgXRtmVUvTVrraqPUokyLHqe".to_string(),
                status: 1,
            },
        ),
        (
            11,
            Wallet {
                min: get_bigint("1024"),
                max: get_bigint("2047"),
                address: "1PgQVLmst3Z314JrQn5TNiys8Hc38TcXJu".to_string(),
                status: 1,
            },
        ),
        (
            12,
            Wallet {
                min: get_bigint("2048"),
                max: get_bigint("4095"),
                address: "1DBaumZxUkM4qMQRt2LVWyFJq5kDtSZQot".to_string(),
                status: 1,
            },
        ),
        (
            13,
            Wallet {
                min: get_bigint("4096"),
                max: get_bigint("8191"),
                address: "1Pie8JkxBT6MGPz9Nvi3fsPkr2D8q3GBc1".to_string(),
                status: 1,
            },
        ),
        (
            14,
            Wallet {
                min: get_bigint("8192"),
                max: get_bigint("16383"),
                address: "1ErZWg5cFCe4Vw5BzgfzB74VNLaXEiEkhk".to_string(),
                status: 1,
            },
        ),
        (
            15,
            Wallet {
                min: get_bigint("16384"),
                max: get_bigint("32767"),
                address: "1QCbW9HWnwQWiQqVo5exhAnmfqKRrCRsvW".to_string(),
                status: 1,
            },
        ),
        (
            16,
            Wallet {
                min: get_bigint("32768"),
                max: get_bigint("65535"),
                address: "1BDyrQ6WoF8VN3g9SAS1iKZcPzFfnDVieY".to_string(),
                status: 1,
            },
        ),
        (
            17,
            Wallet {
                min: get_bigint("65536"),
                max: get_bigint("131071"),
                address: "1HduPEXZRdG26SUT5Yk83mLkPyjnZuJ7Bm".to_string(),
                status: 1,
            },
        ),
        (
            18,
            Wallet {
                min: get_bigint("131072"),
                max: get_bigint("262143"),
                address: "1GnNTmTVLZiqQfLbAdp9DVdicEnB5GoERE".to_string(),
                status: 1,
            },
        ),
        (
            19,
            Wallet {
                min: get_bigint("262144"),
                max: get_bigint("524287"),
                address: "1NWmZRpHH4XSPwsW6dsS3nrNWfL1yrJj4w".to_string(),
                status: 1,
            },
        ),
        (
            20,
            Wallet {
                min: get_bigint("524288"),
                max: get_bigint("1048575"),
                address: "1HsMJxNiV7TLxmoF6uJNkydxPFDog4NQum".to_string(),
                status: 1,
            },
        ),
        (
            21,
            Wallet {
                min: get_bigint("1048576"),
                max: get_bigint("2097151"),
                address: "14oFNXucftsHiUMY8uctg6N487riuyXs4h".to_string(),
                status: 1,
            },
        ),
        (
            22,
            Wallet {
                min: get_bigint("2097152"),
                max: get_bigint("4194303"),
                address: "1CfZWK1QTQE3eS9qn61dQjV89KDjZzfNcv".to_string(),
                status: 1,
            },
        ),
        (
            23,
            Wallet {
                min: get_bigint("4194304"),
                max: get_bigint("8388607"),
                address: "1L2GM8eE7mJWLdo3HZS6su1832NX2txaac".to_string(),
                status: 1,
            },
        ),
        (
            24,
            Wallet {
                min: get_bigint("8388608"),
                max: get_bigint("16777215"),
                address: "1rSnXMr63jdCuegJFuidJqWxUPV7AtUf7".to_string(),
                status: 1,
            },
        ),
        (
            25,
            Wallet {
                min: get_bigint("16777216"),
                max: get_bigint("33554431"),
                address: "15JhYXn6Mx3oF4Y7PcTAv2wVVAuCFFQNiP".to_string(),
                status: 1,
            },
        ),
        (
            26,
            Wallet {
                min: get_bigint("33554432"),
                max: get_bigint("67108863"),
                address: "1JVnST957hGztonaWK6FougdtjxzHzRMMg".to_string(),
                status: 1,
            },
        ),
        (
            27,
            Wallet {
                min: get_bigint("67108864"),
                max: get_bigint("134217727"),
                address: "128z5d7nN7PkCuX5qoA4Ys6pmxUYnEy86k".to_string(),
                status: 1,
            },
        ),
        (
            28,
            Wallet {
                min: get_bigint("134217728"),
                max: get_bigint("268435455"),
                address: "12jbtzBb54r97TCwW3G1gCFoumpckRAPdY".to_string(),
                status: 1,
            },
        ),
        (
            29,
            Wallet {
                min: get_bigint("268435456"),
                max: get_bigint("536870911"),
                address: "19EEC52krRUK1RkUAEZmQdjTyHT7Gp1TYT".to_string(),
                status: 1,
            },
        ),
        (
            30,
            Wallet {
                min: get_bigint("536870912"),
                max: get_bigint("1073741823"),
                address: "1LHtnpd8nU5VHEMkG2TMYYNUjjLc992bps".to_string(),
                status: 1,
            },
        ),
        (
            31,
            Wallet {
                min: get_bigint("1073741824"),
                max: get_bigint("2147483647"),
                address: "1LhE6sCTuGae42Axu1L1ZB7L96yi9irEBE".to_string(),
                status: 1,
            },
        ),
        (
            32,
            Wallet {
                min: get_bigint("2147483648"),
                max: get_bigint("4294967295"),
                address: "1FRoHA9xewq7DjrZ1psWJVeTer8gHRqEvR".to_string(),
                status: 1,
            },
        ),
        (
            33,
            Wallet {
                min: get_bigint("4294967296"),
                max: get_bigint("8589934591"),
                address: "187swFMjz1G54ycVU56B7jZFHFTNVQFDiu".to_string(),
                status: 1,
            },
        ),
        (
            34,
            Wallet {
                min: get_bigint("8589934592"),
                max: get_bigint("17179869183"),
                address: "1PWABE7oUahG2AFFQhhvViQovnCr4rEv7Q".to_string(),
                status: 1,
            },
        ),
        (
            35,
            Wallet {
                min: get_bigint("17179869184"),
                max: get_bigint("34359738367"),
                address: "1PWCx5fovoEaoBowAvF5k91m2Xat9bMgwb".to_string(),
                status: 1,
            },
        ),
        (
            36,
            Wallet {
                min: get_bigint("34359738368"),
                max: get_bigint("68719476735"),
                address: "1Be2UF9NLfyLFbtm3TCbmuocc9N1Kduci1".to_string(),
                status: 1,
            },
        ),
        (
            37,
            Wallet {
                min: get_bigint("68719476736"),
                max: get_bigint("137438953471"),
                address: "14iXhn8bGajVWegZHJ18vJLHhntcpL4dex".to_string(),
                status: 1,
            },
        ),
        (
            38,
            Wallet {
                min: get_bigint("137438953472"),
                max: get_bigint("274877906943"),
                address: "1HBtApAFA9B2YZw3G2YKSMCtb3dVnjuNe2".to_string(),
                status: 1,
            },
        ),
        (
            39,
            Wallet {
                min: get_bigint("274877906944"),
                max: get_bigint("549755813887"),
                address: "122AJhKLEfkFBaGAd84pLp1kfE7xK3GdT8".to_string(),
                status: 1,
            },
        ),
        (
            40,
            Wallet {
                min: get_bigint("549755813888"),
                max: get_bigint("1099511627775"),
                address: "1EeAxcprB2PpCnr34VfZdFrkUWuxyiNEFv".to_string(),
                status: 1,
            },
        ),
        (
            41,
            Wallet {
                min: get_bigint("1099511627776"),
                max: get_bigint("2199023255551"),
                address: "1L5sU9qvJeuwQUdt4y1eiLmquFxKjtHr3E".to_string(),
                status: 1,
            },
        ),
        (
            42,
            Wallet {
                min: get_bigint("2199023255552"),
                max: get_bigint("4398046511103"),
                address: "1E32GPWgDyeyQac4aJxm9HVoLrrEYPnM4N".to_string(),
                status: 1,
            },
        ),
        (
            43,
            Wallet {
                min: get_bigint("4398046511104"),
                max: get_bigint("8796093022207"),
                address: "1PiFuqGpG8yGM5v6rNHWS3TjsG6awgEGA1".to_string(),
                status: 1,
            },
        ),
        (
            44,
            Wallet {
                min: get_bigint("8796093022208"),
                max: get_bigint("17592186044415"),
                address: "1CkR2uS7LmFwc3T2jV8C1BhWb5mQaoxedF".to_string(),
                status: 1,
            },
        ),
        (
            45,
            Wallet {
                min: get_bigint("17592186044416"),
                max: get_bigint("35184372088831"),
                address: "1NtiLNGegHWE3Mp9g2JPkgx6wUg4TW7bbk".to_string(),
                status: 1,
            },
        ),
        (
            46,
            Wallet {
                min: get_bigint("35184372088832"),
                max: get_bigint("70368744177663"),
                address: "1F3JRMWudBaj48EhwcHDdpeuy2jwACNxjP".to_string(),
                status: 1,
            },
        ),
        (
            47,
            Wallet {
                min: get_bigint("70368744177664"),
                max: get_bigint("140737488355327"),
                address: "1Pd8VvT49sHKsmqrQiP61RsVwmXCZ6ay7Z".to_string(),
                status: 1,
            },
        ),
        (
            48,
            Wallet {
                min: get_bigint("140737488355328"),
                max: get_bigint("281474976710655"),
                address: "1DFYhaB2J9q1LLZJWKTnscPWos9VBqDHzv".to_string(),
                status: 1,
            },
        ),
        (
            49,
            Wallet {
                min: get_bigint("281474976710656"),
                max: get_bigint("562949953421311"),
                address: "12CiUhYVTTH33w3SPUBqcpMoqnApAV4WCF".to_string(),
                status: 1,
            },
        ),
        (
            50,
            Wallet {
                min: get_bigint("562949953421312"),
                max: get_bigint("1125899906842623"),
                address: "1MEzite4ReNuWaL5Ds17ePKt2dCxWEofwk".to_string(),
                status: 1,
            },
        ),
        (
            51,
            Wallet {
                min: get_bigint("1125899906842624"),
                max: get_bigint("2251799813685247"),
                address: "1NpnQyZ7x24ud82b7WiRNvPm6N8bqGQnaS".to_string(),
                status: 1,
            },
        ),
        (
            52,
            Wallet {
                min: get_bigint("2251799813685248"),
                max: get_bigint("4503599627370495"),
                address: "15z9c9sVpu6fwNiK7dMAFgMYSK4GqsGZim".to_string(),
                status: 1,
            },
        ),
        (
            53,
            Wallet {
                min: get_bigint("4503599627370496"),
                max: get_bigint("9007199254740991"),
                address: "15K1YKJMiJ4fpesTVUcByoz334rHmknxmT".to_string(),
                status: 1,
            },
        ),
        (
            54,
            Wallet {
                min: get_bigint("9007199254740992"),
                max: get_bigint("18014398509481983"),
                address: "1KYUv7nSvXx4642TKeuC2SNdTk326uUpFy".to_string(),
                status: 1,
            },
        ),
        (
            55,
            Wallet {
                min: get_bigint("18014398509481984"),
                max: get_bigint("36028797018963967"),
                address: "1LzhS3k3e9Ub8i2W1V8xQFdB8n2MYCHPCa".to_string(),
                status: 1,
            },
        ),
        (
            56,
            Wallet {
                min: get_bigint("36028797018963968"),
                max: get_bigint("72057594037927935"),
                address: "17aPYR1m6pVAacXg1PTDDU7XafvK1dxvhi".to_string(),
                status: 1,
            },
        ),
        (
            57,
            Wallet {
                min: get_bigint("72057594037927936"),
                max: get_bigint("144115188075855871"),
                address: "15c9mPGLku1HuW9LRtBf4jcHVpBUt8txKz".to_string(),
                status: 1,
            },
        ),
        (
            58,
            Wallet {
                min: get_bigint("144115188075855872"),
                max: get_bigint("288230376151711743"),
                address: "1Dn8NF8qDyyfHMktmuoQLGyjWmZXgvosXf".to_string(),
                status: 1,
            },
        ),
        (
            59,
            Wallet {
                min: get_bigint("288230376151711744"),
                max: get_bigint("576460752303423487"),
                address: "1HAX2n9Uruu9YDt4cqRgYcvtGvZj1rbUyt".to_string(),
                status: 1,
            },
        ),
        (
            60,
            Wallet {
                min: get_bigint("576460752303423488"),
                max: get_bigint("1152921504606846975"),
                address: "1Kn5h2qpgw9mWE5jKpk8PP4qvvJ1QVy8su".to_string(),
                status: 1,
            },
        ),
        (
            61,
            Wallet {
                min: get_bigint("1152921504606846976"),
                max: get_bigint("2305843009213693951"),
                address: "1AVJKwzs9AskraJLGHAZPiaZcrpDr1U6AB".to_string(),
                status: 1,
            },
        ),
        (
            62,
            Wallet {
                min: get_bigint("2305843009213693952"),
                max: get_bigint("4611686018427387903"),
                address: "1Me6EfpwZK5kQziBwBfvLiHjaPGxCKLoJi".to_string(),
                status: 1,
            },
        ),
        (
            63,
            Wallet {
                min: get_bigint("4611686018427387904"),
                max: get_bigint("9223372036854775807"),
                address: "1NpYjtLira16LfGbGwZJ5JbDPh3ai9bjf4".to_string(),
                status: 1,
            },
        ),
        (
            64,
            Wallet {
                min: get_bigint("9223372036854775808"),
                max: get_bigint("18446744073709551615"),
                address: "16jY7qLJnxb7CHZyqBP8qca9d51gAjyXQN".to_string(),
                status: 1,
            },
        ),
        (
            65,
            Wallet {
                min: get_bigint("18446744073709551616"),
                max: get_bigint("36893488147419103231"),
                address: "18ZMbwUFLMHoZBbfpCjUJQTCMCbktshgpe".to_string(),
                status: 1,
            },
        ),
        (
            66,
            Wallet {
                min: get_bigint("36893488147419103232"),
                max: get_bigint("73786976294838206463"),
                address: "13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so".to_string(),
                status: 1,
            },
        ),
        (
            67,
            Wallet {
                min: get_bigint("73786976294838206464"),
                max: get_bigint("147573952589676412927"),
                address: "1BY8GQbnueYofwSuFAT3USAhGjPrkxDdW9".to_string(),
                status: 1,
            },
        ),
        (
            68,
            Wallet {
                min: get_bigint("147573952589676412928"),
                max: get_bigint("295147905179352825855"),
                address: "1MVDYgVaSN6iKKEsbzRUAYFrYJadLYZvvZ".to_string(),
                status: 1,
            },
        ),
        (
            69,
            Wallet {
                min: get_bigint("295147905179352825856"),
                max: get_bigint("590295810358705651711"),
                address: "19vkiEajfhuZ8bs8Zu2jgmC6oqZbWqhxhG".to_string(),
                status: 1,
            },
        ),
        (
            70,
            Wallet {
                min: get_bigint("590295810358705651712"),
                max: get_bigint("1180591620717411303423"),
                address: "19YZECXj3SxEZMoUeJ1yiPsw8xANe7M7QR".to_string(),
                status: 1,
            },
        ),
        (
            71,
            Wallet {
                min: get_bigint("1180591620717411303424"),
                max: get_bigint("2361183241434822606847"),
                address: "1PWo3JeB9jrGwfHDNpdGK54CRas7fsVzXU".to_string(),
                status: 1,
            },
        ),
        (
            72,
            Wallet {
                min: get_bigint("2361183241434822606848"),
                max: get_bigint("4722366482869645213695"),
                address: "1JTK7s9YVYywfm5XUH7RNhHJH1LshCaRFR".to_string(),
                status: 1,
            },
        ),
        (
            73,
            Wallet {
                min: get_bigint("4722366482869645213696"),
                max: get_bigint("9444732965739290427391"),
                address: "12VVRNPi4SJqUTsp6FmqDqY5sGosDtysn4".to_string(),
                status: 1,
            },
        ),
        (
            74,
            Wallet {
                min: get_bigint("9444732965739290427392"),
                max: get_bigint("18889465931478580854783"),
                address: "1FWGcVDK3JGzCC3WtkYetULPszMaK2Jksv".to_string(),
                status: 1,
            },
        ),
        (
            75,
            Wallet {
                min: get_bigint("18889465931478580854784"),
                max: get_bigint("37778931862957161709567"),
                address: "1J36UjUByGroXcCvmj13U6uwaVv9caEeAt".to_string(),
                status: 1,
            },
        ),
        (
            76,
            Wallet {
                min: get_bigint("37778931862957161709568"),
                max: get_bigint("75557863725914323419135"),
                address: "1DJh2eHFYQfACPmrvpyWc8MSTYKh7w9eRF".to_string(),
                status: 1,
            },
        ),
        (
            77,
            Wallet {
                min: get_bigint("75557863725914323419136"),
                max: get_bigint("151115727451828646838271"),
                address: "1Bxk4CQdqL9p22JEtDfdXMsng1XacifUtE".to_string(),
                status: 1,
            },
        ),
        (
            78,
            Wallet {
                min: get_bigint("151115727451828646838272"),
                max: get_bigint("302231454903657293676543"),
                address: "15qF6X51huDjqTmF9BJgxXdt1xcj46Jmhb".to_string(),
                status: 1,
            },
        ),
        (
            79,
            Wallet {
                min: get_bigint("302231454903657293676544"),
                max: get_bigint("604462909807314587353087"),
                address: "1ARk8HWJMn8js8tQmGUJeQHjSE7KRkn2t8".to_string(),
                status: 1,
            },
        ),
        (
            80,
            Wallet {
                min: get_bigint("604462909807314587353088"),
                max: get_bigint("1208925819614629174706175"),
                address: "1BCf6rHUW6m3iH2ptsvnjgLruAiPQQepLe".to_string(),
                status: 1,
            },
        ),
        (
            81,
            Wallet {
                min: get_bigint("1208925819614629174706176"),
                max: get_bigint("2417851639229258349412351"),
                address: "15qsCm78whspNQFydGJQk5rexzxTQopnHZ".to_string(),
                status: 1,
            },
        ),
        (
            82,
            Wallet {
                min: get_bigint("2417851639229258349412352"),
                max: get_bigint("4835703278458516698824703"),
                address: "13zYrYhhJxp6Ui1VV7pqa5WDhNWM45ARAC".to_string(),
                status: 1,
            },
        ),
        (
            83,
            Wallet {
                min: get_bigint("4835703278458516698824704"),
                max: get_bigint("9671406556917033397649407"),
                address: "14MdEb4eFcT3MVG5sPFG4jGLuHJSnt1Dk2".to_string(),
                status: 1,
            },
        ),
        (
            84,
            Wallet {
                min: get_bigint("9671406556917033397649408"),
                max: get_bigint("19342813113834066795298815"),
                address: "1CMq3SvFcVEcpLMuuH8PUcNiqsK1oicG2D".to_string(),
                status: 1,
            },
        ),
        (
            85,
            Wallet {
                min: get_bigint("19342813113834066795298816"),
                max: get_bigint("38685626227668133590597631"),
                address: "1Kh22PvXERd2xpTQk3ur6pPEqFeckCJfAr".to_string(),
                status: 1,
            },
        ),
        (
            86,
            Wallet {
                min: get_bigint("38685626227668133590597632"),
                max: get_bigint("77371252455336267181195263"),
                address: "1K3x5L6G57Y494fDqBfrojD28UJv4s5JcK".to_string(),
                status: 1,
            },
        ),
        (
            87,
            Wallet {
                min: get_bigint("77371252455336267181195264"),
                max: get_bigint("154742504910672534362390527"),
                address: "1PxH3K1Shdjb7gSEoTX7UPDZ6SH4qGPrvq".to_string(),
                status: 1,
            },
        ),
        (
            88,
            Wallet {
                min: get_bigint("154742504910672534362390528"),
                max: get_bigint("309485009821345068724781055"),
                address: "16AbnZjZZipwHMkYKBSfswGWKDmXHjEpSf".to_string(),
                status: 1,
            },
        ),
        (
            89,
            Wallet {
                min: get_bigint("309485009821345068724781056"),
                max: get_bigint("618970019642690137449562111"),
                address: "19QciEHbGVNY4hrhfKXmcBBCrJSBZ6TaVt".to_string(),
                status: 1,
            },
        ),
        (
            90,
            Wallet {
                min: get_bigint("618970019642690137449562112"),
                max: get_bigint("1237940039285380274899124223"),
                address: "1L12FHH2FHjvTviyanuiFVfmzCy46RRATU".to_string(),
                status: 1,
            },
        ),
        (
            91,
            Wallet {
                min: get_bigint("1237940039285380274899124224"),
                max: get_bigint("2475880078570760549798248447"),
                address: "1EzVHtmbN4fs4MiNk3ppEnKKhsmXYJ4s74".to_string(),
                status: 1,
            },
        ),
        (
            92,
            Wallet {
                min: get_bigint("2475880078570760549798248448"),
                max: get_bigint("4951760157141521099596496895"),
                address: "1AE8NzzgKE7Yhz7BWtAcAAxiFMbPo82NB5".to_string(),
                status: 1,
            },
        ),
        (
            93,
            Wallet {
                min: get_bigint("4951760157141521099596496896"),
                max: get_bigint("9903520314283042199192993791"),
                address: "17Q7tuG2JwFFU9rXVj3uZqRtioH3mx2Jad".to_string(),
                status: 1,
            },
        ),
        (
            94,
            Wallet {
                min: get_bigint("9903520314283042199192993792"),
                max: get_bigint("19807040628566084398385987583"),
                address: "1K6xGMUbs6ZTXBnhw1pippqwK6wjBWtNpL".to_string(),
                status: 1,
            },
        ),
        (
            95,
            Wallet {
                min: get_bigint("19807040628566084398385987584"),
                max: get_bigint("39614081257132168796771975167"),
                address: "19eVSDuizydXxhohGh8Ki9WY9KsHdSwoQC".to_string(),
                status: 1,
            },
        ),
        (
            96,
            Wallet {
                min: get_bigint("39614081257132168796771975168"),
                max: get_bigint("79228162514264337593543950335"),
                address: "15ANYzzCp5BFHcCnVFzXqyibpzgPLWaD8b".to_string(),
                status: 1,
            },
        ),
        (
            97,
            Wallet {
                min: get_bigint("79228162514264337593543950336"),
                max: get_bigint("158456325028528675187087900671"),
                address: "18ywPwj39nGjqBrQJSzZVq2izR12MDpDr8".to_string(),
                status: 1,
            },
        ),
        (
            98,
            Wallet {
                min: get_bigint("158456325028528675187087900672"),
                max: get_bigint("316912650057057350374175801343"),
                address: "1CaBVPrwUxbQYYswu32w7Mj4HR4maNoJSX".to_string(),
                status: 1,
            },
        ),
        (
            99,
            Wallet {
                min: get_bigint("316912650057057350374175801344"),
                max: get_bigint("633825300114114700748351602687"),
                address: "1JWnE6p6UN7ZJBN7TtcbNDoRcjFtuDWoNL".to_string(),
                status: 1,
            },
        ),
        (
            100,
            Wallet {
                min: get_bigint("633825300114114700748351602688"),
                max: get_bigint("1267650600228229401496703205375"),
                address: "1KCgMv8fo2TPBpddVi9jqmMmcne9uSNJ5F".to_string(),
                status: 1,
            },
        ),
        (
            101,
            Wallet {
                min: get_bigint("1267650600228229401496703205376"),
                max: get_bigint("2535301200456458802993406410751"),
                address: "1CKCVdbDJasYmhswB6HKZHEAnNaDpK7W4n".to_string(),
                status: 1,
            },
        ),
        (
            102,
            Wallet {
                min: get_bigint("2535301200456458802993406410752"),
                max: get_bigint("5070602400912917605986812821503"),
                address: "1PXv28YxmYMaB8zxrKeZBW8dt2HK7RkRPX".to_string(),
                status: 1,
            },
        ),
        (
            103,
            Wallet {
                min: get_bigint("5070602400912917605986812821504"),
                max: get_bigint("10141204801825835211973625643007"),
                address: "1AcAmB6jmtU6AiEcXkmiNE9TNVPsj9DULf".to_string(),
                status: 1,
            },
        ),
        (
            104,
            Wallet {
                min: get_bigint("10141204801825835211973625643008"),
                max: get_bigint("20282409603651670423947251286015"),
                address: "1EQJvpsmhazYCcKX5Au6AZmZKRnzarMVZu".to_string(),
                status: 1,
            },
        ),
        (
            105,
            Wallet {
                min: get_bigint("20282409603651670423947251286016"),
                max: get_bigint("40564819207303340847894502572031"),
                address: "1CMjscKB3QW7SDyQ4c3C3DEUHiHRhiZVib".to_string(),
                status: 1,
            },
        ),
        (
            106,
            Wallet {
                min: get_bigint("40564819207303340847894502572032"),
                max: get_bigint("81129638414606681695789005144063"),
                address: "18KsfuHuzQaBTNLASyj15hy4LuqPUo1FNB".to_string(),
                status: 1,
            },
        ),
        (
            107,
            Wallet {
                min: get_bigint("81129638414606681695789005144064"),
                max: get_bigint("162259276829213363391578010288127"),
                address: "15EJFC5ZTs9nhsdvSUeBXjLAuYq3SWaxTc".to_string(),
                status: 1,
            },
        ),
        (
            108,
            Wallet {
                min: get_bigint("162259276829213363391578010288128"),
                max: get_bigint("324518553658426726783156020576255"),
                address: "1HB1iKUqeffnVsvQsbpC6dNi1XKbyNuqao".to_string(),
                status: 1,
            },
        ),
        (
            109,
            Wallet {
                min: get_bigint("324518553658426726783156020576256"),
                max: get_bigint("649037107316853453566312041152511"),
                address: "1GvgAXVCbA8FBjXfWiAms4ytFeJcKsoyhL".to_string(),
                status: 1,
            },
        ),
        (
            110,
            Wallet {
                min: get_bigint("649037107316853453566312041152512"),
                max: get_bigint("1298074214633706907132624082305023"),
                address: "12JzYkkN76xkwvcPT6AWKZtGX6w2LAgsJg".to_string(),
                status: 1,
            },
        ),
        (
            111,
            Wallet {
                min: get_bigint("1298074214633706907132624082305024"),
                max: get_bigint("2596148429267413814265248164610047"),
                address: "1824ZJQ7nKJ9QFTRBqn7z7dHV5EGpzUpH3".to_string(),
                status: 1,
            },
        ),
        (
            112,
            Wallet {
                min: get_bigint("2596148429267413814265248164610048"),
                max: get_bigint("5192296858534827628530496329220095"),
                address: "18A7NA9FTsnJxWgkoFfPAFbQzuQxpRtCos".to_string(),
                status: 1,
            },
        ),
        (
            113,
            Wallet {
                min: get_bigint("5192296858534827628530496329220096"),
                max: get_bigint("10384593717069655257060992658440191"),
                address: "1NeGn21dUDDeqFQ63xb2SpgUuXuBLA4WT4".to_string(),
                status: 1,
            },
        ),
        (
            114,
            Wallet {
                min: get_bigint("10384593717069655257060992658440192"),
                max: get_bigint("20769187434139310514121985316880383"),
                address: "174SNxfqpdMGYy5YQcfLbSTK3MRNZEePoy".to_string(),
                status: 1,
            },
        ),
        (
            115,
            Wallet {
                min: get_bigint("20769187434139310514121985316880384"),
                max: get_bigint("41538374868278621028243970633760767"),
                address: "1NLbHuJebVwUZ1XqDjsAyfTRUPwDQbemfv".to_string(),
                status: 1,
            },
        ),
        (
            116,
            Wallet {
                min: get_bigint("41538374868278621028243970633760768"),
                max: get_bigint("83076749736557242056487941267521535"),
                address: "1MnJ6hdhvK37VLmqcdEwqC3iFxyWH2PHUV".to_string(),
                status: 1,
            },
        ),
        (
            117,
            Wallet {
                min: get_bigint("83076749736557242056487941267521536"),
                max: get_bigint("166153499473114484112975882535043071"),
                address: "1KNRfGWw7Q9Rmwsc6NT5zsdvEb9M2Wkj5Z".to_string(),
                status: 1,
            },
        ),
        (
            118,
            Wallet {
                min: get_bigint("166153499473114484112975882535043072"),
                max: get_bigint("332306998946228968225951765070086143"),
                address: "1PJZPzvGX19a7twf5HyD2VvNiPdHLzm9F6".to_string(),
                status: 1,
            },
        ),
        (
            119,
            Wallet {
                min: get_bigint("332306998946228968225951765070086144"),
                max: get_bigint("664613997892457936451903530140172287"),
                address: "1GuBBhf61rnvRe4K8zu8vdQB3kHzwFqSy7".to_string(),
                status: 1,
            },
        ),
        (
            120,
            Wallet {
                min: get_bigint("664613997892457936451903530140172288"),
                max: get_bigint("1329227995784915872903807060280344575"),
                address: "17s2b9ksz5y7abUm92cHwG8jEPCzK3dLnT".to_string(),
                status: 1,
            },
        ),
        (
            121,
            Wallet {
                min: get_bigint("1329227995784915872903807060280344576"),
                max: get_bigint("2658455991569831745807614120560689151"),
                address: "1GDSuiThEV64c166LUFC9uDcVdGjqkxKyh".to_string(),
                status: 1,
            },
        ),
        (
            122,
            Wallet {
                min: get_bigint("2658455991569831745807614120560689152"),
                max: get_bigint("5316911983139663491615228241121378303"),
                address: "1Me3ASYt5JCTAK2XaC32RMeH34PdprrfDx".to_string(),
                status: 1,
            },
        ),
        (
            123,
            Wallet {
                min: get_bigint("5316911983139663491615228241121378304"),
                max: get_bigint("10633823966279326983230456482242756607"),
                address: "1CdufMQL892A69KXgv6UNBD17ywWqYpKut".to_string(),
                status: 1,
            },
        ),
        (
            124,
            Wallet {
                min: get_bigint("10633823966279326983230456482242756608"),
                max: get_bigint("21267647932558653966460912964485513215"),
                address: "1BkkGsX9ZM6iwL3zbqs7HWBV7SvosR6m8N".to_string(),
                status: 1,
            },
        ),
        (
            125,
            Wallet {
                min: get_bigint("21267647932558653966460912964485513216"),
                max: get_bigint("42535295865117307932921825928971026431"),
                address: "1PXAyUB8ZoH3WD8n5zoAthYjN15yN5CVq5".to_string(),
                status: 1,
            },
        ),
        (
            126,
            Wallet {
                min: get_bigint("42535295865117307932921825928971026432"),
                max: get_bigint("85070591730234615865843651857942052863"),
                address: "1AWCLZAjKbV1P7AHvaPNCKiB7ZWVDMxFiz".to_string(),
                status: 1,
            },
        ),
        (
            127,
            Wallet {
                min: get_bigint("85070591730234615865843651857942052864"),
                max: get_bigint("170141183460469231731687303715884105727"),
                address: "1G6EFyBRU86sThN3SSt3GrHu1sA7w7nzi4".to_string(),
                status: 1,
            },
        ),
        (
            128,
            Wallet {
                min: get_bigint("170141183460469231731687303715884105728"),
                max: get_bigint("340282366920938463463374607431768211455"),
                address: "1MZ2L1gFrCtkkn6DnTT2e4PFUTHw9gNwaj".to_string(),
                status: 1,
            },
        ),
        (
            129,
            Wallet {
                min: get_bigint("340282366920938463463374607431768211456"),
                max: get_bigint("680564733841876926926749214863536422911"),
                address: "1Hz3uv3nNZzBVMXLGadCucgjiCs5W9vaGz".to_string(),
                status: 1,
            },
        ),
        (
            130,
            Wallet {
                min: get_bigint("680564733841876926926749214863536422912"),
                max: get_bigint("1361129467683753853853498429727072845823"),
                address: "1Fo65aKq8s8iquMt6weF1rku1moWVEd5Ua".to_string(),
                status: 1,
            },
        ),
        (
            131,
            Wallet {
                min: get_bigint("1361129467683753853853498429727072845824"),
                max: get_bigint("2722258935367507707706996859454145691647"),
                address: "16zRPnT8znwq42q7XeMkZUhb1bKqgRogyy".to_string(),
                status: 1,
            },
        ),
        (
            132,
            Wallet {
                min: get_bigint("2722258935367507707706996859454145691648"),
                max: get_bigint("5444517870735015415413993718908291383295"),
                address: "1KrU4dHE5WrW8rhWDsTRjR21r8t3dsrS3R".to_string(),
                status: 1,
            },
        ),
        (
            133,
            Wallet {
                min: get_bigint("5444517870735015415413993718908291383296"),
                max: get_bigint("10889035741470030830827987437816582766591"),
                address: "17uDfp5r4n441xkgLFmhNoSW1KWp6xVLD".to_string(),
                status: 1,
            },
        ),
        (
            134,
            Wallet {
                min: get_bigint("10889035741470030830827987437816582766592"),
                max: get_bigint("21778071482940061661655974875633165533183"),
                address: "13A3JrvXmvg5w9XGvyyR4JEJqiLz8ZySY3".to_string(),
                status: 1,
            },
        ),
        (
            135,
            Wallet {
                min: get_bigint("21778071482940061661655974875633165533184"),
                max: get_bigint("43556142965880123323311949751266331066367"),
                address: "16RGFo6hjq9ym6Pj7N5H7L1NR1rVPJyw2v".to_string(),
                status: 1,
            },
        ),
        (
            136,
            Wallet {
                min: get_bigint("43556142965880123323311949751266331066368"),
                max: get_bigint("87112285931760246646623899502532662132735"),
                address: "1UDHPdovvR985NrWSkdWQDEQ1xuRiTALq".to_string(),
                status: 1,
            },
        ),
        (
            137,
            Wallet {
                min: get_bigint("87112285931760246646623899502532662132736"),
                max: get_bigint("174224571863520493293247799005065324265471"),
                address: "15nf31J46iLuK1ZkTnqHo7WgN5cARFK3RA".to_string(),
                status: 1,
            },
        ),
        (
            138,
            Wallet {
                min: get_bigint("174224571863520493293247799005065324265472"),
                max: get_bigint("348449143727040986586495598010130648530943"),
                address: "1Ab4vzG6wEQBDNQM1B2bvUz4fqXXdFk2WT".to_string(),
                status: 1,
            },
        ),
        (
            139,
            Wallet {
                min: get_bigint("348449143727040986586495598010130648530944"),
                max: get_bigint("696898287454081973172991196020261297061887"),
                address: "1Fz63c775VV9fNyj25d9Xfw3YHE6sKCxbt".to_string(),
                status: 1,
            },
        ),
        (
            140,
            Wallet {
                min: get_bigint("696898287454081973172991196020261297061888"),
                max: get_bigint("1393796574908163946345982392040522594123775"),
                address: "1QKBaU6WAeycb3DbKbLBkX7vJiaS8r42Xo".to_string(),
                status: 1,
            },
        ),
        (
            141,
            Wallet {
                min: get_bigint("1393796574908163946345982392040522594123776"),
                max: get_bigint("2787593149816327892691964784081045188247551"),
                address: "1CD91Vm97mLQvXhrnoMChhJx4TP9MaQkJo".to_string(),
                status: 1,
            },
        ),
        (
            142,
            Wallet {
                min: get_bigint("2787593149816327892691964784081045188247552"),
                max: get_bigint("5575186299632655785383929568162090376495103"),
                address: "15MnK2jXPqTMURX4xC3h4mAZxyCcaWWEDD".to_string(),
                status: 1,
            },
        ),
        (
            143,
            Wallet {
                min: get_bigint("5575186299632655785383929568162090376495104"),
                max: get_bigint("11150372599265311570767859136324180752990207"),
                address: "13N66gCzWWHEZBxhVxG18P8wyjEWF9Yoi1".to_string(),
                status: 1,
            },
        ),
        (
            144,
            Wallet {
                min: get_bigint("11150372599265311570767859136324180752990208"),
                max: get_bigint("22300745198530623141535718272648361505980415"),
                address: "1NevxKDYuDcCh1ZMMi6ftmWwGrZKC6j7Ux".to_string(),
                status: 1,
            },
        ),
        (
            145,
            Wallet {
                min: get_bigint("22300745198530623141535718272648361505980416"),
                max: get_bigint("44601490397061246283071436545296723011960831"),
                address: "19GpszRNUej5yYqxXoLnbZWKew3KdVLkXg".to_string(),
                status: 1,
            },
        ),
        (
            146,
            Wallet {
                min: get_bigint("44601490397061246283071436545296723011960832"),
                max: get_bigint("89202980794122492566142873090593446023921663"),
                address: "1M7ipcdYHey2Y5RZM34MBbpugghmjaV89P".to_string(),
                status: 1,
            },
        ),
        (
            147,
            Wallet {
                min: get_bigint("89202980794122492566142873090593446023921664"),
                max: get_bigint("178405961588244985132285746181186892047843327"),
                address: "18aNhurEAJsw6BAgtANpexk5ob1aGTwSeL".to_string(),
                status: 1,
            },
        ),
        (
            148,
            Wallet {
                min: get_bigint("178405961588244985132285746181186892047843328"),
                max: get_bigint("356811923176489970264571492362373784095686655"),
                address: "1FwZXt6EpRT7Fkndzv6K4b4DFoT4trbMrV".to_string(),
                status: 1,
            },
        ),
        (
            149,
            Wallet {
                min: get_bigint("356811923176489970264571492362373784095686656"),
                max: get_bigint("713623846352979940529142984724747568191373311"),
                address: "1CXvTzR6qv8wJ7eprzUKeWxyGcHwDYP1i2".to_string(),
                status: 1,
            },
        ),
        (
            150,
            Wallet {
                min: get_bigint("713623846352979940529142984724747568191373312"),
                max: get_bigint("1427247692705959881058285969449495136382746623"),
                address: "1MUJSJYtGPVGkBCTqGspnxyHahpt5Te8jy".to_string(),
                status: 1,
            },
        ),
        (
            151,
            Wallet {
                min: get_bigint("1427247692705959881058285969449495136382746624"),
                max: get_bigint("2854495385411919762116571938898990272765493247"),
                address: "13Q84TNNvgcL3HJiqQPvyBb9m4hxjS3jkV".to_string(),
                status: 1,
            },
        ),
        (
            152,
            Wallet {
                min: get_bigint("2854495385411919762116571938898990272765493248"),
                max: get_bigint("5708990770823839524233143877797980545530986495"),
                address: "1LuUHyrQr8PKSvbcY1v1PiuGuqFjWpDumN".to_string(),
                status: 1,
            },
        ),
        (
            153,
            Wallet {
                min: get_bigint("5708990770823839524233143877797980545530986496"),
                max: get_bigint("11417981541647679048466287755595961091061972991"),
                address: "18192XpzzdDi2K11QVHR7td2HcPS6Qs5vg".to_string(),
                status: 1,
            },
        ),
        (
            154,
            Wallet {
                min: get_bigint("11417981541647679048466287755595961091061972992"),
                max: get_bigint("22835963083295358096932575511191922182123945983"),
                address: "1NgVmsCCJaKLzGyKLFJfVequnFW9ZvnMLN".to_string(),
                status: 1,
            },
        ),
        (
            155,
            Wallet {
                min: get_bigint("22835963083295358096932575511191922182123945984"),
                max: get_bigint("45671926166590716193865151022383844364247891967"),
                address: "1AoeP37TmHdFh8uN72fu9AqgtLrUwcv2wJ".to_string(),
                status: 1,
            },
        ),
        (
            156,
            Wallet {
                min: get_bigint("45671926166590716193865151022383844364247891968"),
                max: get_bigint("91343852333181432387730302044767688728495783935"),
                address: "1FTpAbQa4h8trvhQXjXnmNhqdiGBd1oraE".to_string(),
                status: 1,
            },
        ),
        (
            157,
            Wallet {
                min: get_bigint("91343852333181432387730302044767688728495783936"),
                max: get_bigint("182687704666362864775460604089535377456991567871"),
                address: "14JHoRAdmJg3XR4RjMDh6Wed6ft6hzbQe9".to_string(),
                status: 1,
            },
        ),
        (
            158,
            Wallet {
                min: get_bigint("182687704666362864775460604089535377456991567872"),
                max: get_bigint("365375409332725729550921208179070754913983135743"),
                address: "19z6waranEf8CcP8FqNgdwUe1QRxvUNKBG".to_string(),
                status: 1,
            },
        ),
        (
            159,
            Wallet {
                min: get_bigint("365375409332725729550921208179070754913983135744"),
                max: get_bigint("730750818665451459101842416358141509827966271487"),
                address: "14u4nA5sugaswb6SZgn5av2vuChdMnD9E5".to_string(),
                status: 1,
            },
        ),
        (
            160,
            Wallet {
                min: get_bigint("730750818665451459101842416358141509827966271488"),
                max: get_bigint("1461501637330902918203684832716283019655932542975"),
                address: "1NBC8uXJy1GiJ6drkiZa1WuKn51ps7EPTv".to_string(),
                status: 1,
            },
        ),
        (
            161,
            Wallet {
                min: get_bigint("1461501637330902918203684832716283019655932542976"),
                max: get_bigint("2923003274661805836407369665432566039311865085951"),
                address: "18bHfcm8kGoAhBaQXzzVcG5534mdpWK981".to_string(),
                status: 1,
            },
        ),
    ])
}
