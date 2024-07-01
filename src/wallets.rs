use std::collections::BTreeMap;
use super::wallet::Wallet;


pub fn wallets() -> BTreeMap<u8, Wallet> {
    BTreeMap::from([
        (
            1,
            Wallet {
                min: "0x1".to_string(),
                max: "0x1".to_string(),
                address: "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH".to_string(),
                status: 1,
            },
        ),
        (
            2,
            Wallet {
                min: "0x2".to_string(),
                max: "0x3".to_string(),
                address: "1CUNEBjYrCn2y1SdiUMohaKUi4wpP326Lb".to_string(),
                status: 1,
            },
        ),
        (
            3,
            Wallet {
                min: "0x4".to_string(),
                max: "0x7".to_string(),
                address: "19ZewH8Kk1PDbSNdJ97FP4EiCjTRaZMZQA".to_string(),
                status: 1,
            },
        ),
        (
            4,
            Wallet {
                min: "0x8".to_string(),
                max: "0xf".to_string(),
                address: "1EhqbyUMvvs7BfL8goY6qcPbD6YKfPqb7e".to_string(),
                status: 1,
            },
        ),
        (
            5,
            Wallet {
                min: "0x10".to_string(),
                max: "0x1f".to_string(),
                address: "1E6NuFjCi27W5zoXg8TRdcSRq84zJeBW3k".to_string(),
                status: 1,
            },
        ),
        (
            6,
            Wallet {
                min: "0x20".to_string(),
                max: "0x3f".to_string(),
                address: "1PitScNLyp2HCygzadCh7FveTnfmpPbfp8".to_string(),
                status: 1,
            },
        ),
        (
            7,
            Wallet {
                min: "0x40".to_string(),
                max: "0x7f".to_string(),
                address: "1McVt1vMtCC7yn5b9wgX1833yCcLXzueeC".to_string(),
                status: 1,
            },
        ),
        (
            8,
            Wallet {
                min: "0x80".to_string(),
                max: "0xff".to_string(),
                address: "1M92tSqNmQLYw33fuBvjmeadirh1ysMBxK".to_string(),
                status: 1,
            },
        ),
        (
            9,
            Wallet {
                min: "0x100".to_string(),
                max: "0x1ff".to_string(),
                address: "1CQFwcjw1dwhtkVWBttNLDtqL7ivBonGPV".to_string(),
                status: 1,
            },
        ),
        (
            10,
            Wallet {
                min: "0x200".to_string(),
                max: "0x3ff".to_string(),
                address: "1LeBZP5QCwwgXRtmVUvTVrraqPUokyLHqe".to_string(),
                status: 1,
            },
        ),
        (
            11,
            Wallet {
                min: "0x400".to_string(),
                max: "0x7ff".to_string(),
                address: "1PgQVLmst3Z314JrQn5TNiys8Hc38TcXJu".to_string(),
                status: 1,
            },
        ),
        (
            12,
            Wallet {
                min: "0x800".to_string(),
                max: "0xfff".to_string(),
                address: "1DBaumZxUkM4qMQRt2LVWyFJq5kDtSZQot".to_string(),
                status: 1,
            },
        ),
        (
            13,
            Wallet {
                min: "0x1000".to_string(),
                max: "0x1fff".to_string(),
                address: "1Pie8JkxBT6MGPz9Nvi3fsPkr2D8q3GBc1".to_string(),
                status: 1,
            },
        ),
        (
            14,
            Wallet {
                min: "0x2000".to_string(),
                max: "0x3fff".to_string(),
                address: "1ErZWg5cFCe4Vw5BzgfzB74VNLaXEiEkhk".to_string(),
                status: 1,
            },
        ),
        (
            15,
            Wallet {
                min: "0x4000".to_string(),
                max: "0x7fff".to_string(),
                address: "1QCbW9HWnwQWiQqVo5exhAnmfqKRrCRsvW".to_string(),
                status: 1,
            },
        ),
        (
            16,
            Wallet {
                min: "0x8000".to_string(),
                max: "0xffff".to_string(),
                address: "1BDyrQ6WoF8VN3g9SAS1iKZcPzFfnDVieY".to_string(),
                status: 1,
            },
        ),
        (
            17,
            Wallet {
                min: "0x10000".to_string(),
                max: "0x1ffff".to_string(),
                address: "1HduPEXZRdG26SUT5Yk83mLkPyjnZuJ7Bm".to_string(),
                status: 1,
            },
        ),
        (
            18,
            Wallet {
                min: "0x20000".to_string(),
                max: "0x3ffff".to_string(),
                address: "1GnNTmTVLZiqQfLbAdp9DVdicEnB5GoERE".to_string(),
                status: 1,
            },
        ),
        (
            19,
            Wallet {
                min: "0x40000".to_string(),
                max: "0x7ffff".to_string(),
                address: "1NWmZRpHH4XSPwsW6dsS3nrNWfL1yrJj4w".to_string(),
                status: 1,
            },
        ),
        (
            20,
            Wallet {
                min: "0x80000".to_string(),
                max: "0xfffff".to_string(),
                address: "1HsMJxNiV7TLxmoF6uJNkydxPFDog4NQum".to_string(),
                status: 1,
            },
        ),
        (
            21,
            Wallet {
                min: "0x100000".to_string(),
                max: "0x1fffff".to_string(),
                address: "14oFNXucftsHiUMY8uctg6N487riuyXs4h".to_string(),
                status: 1,
            },
        ),
        (
            22,
            Wallet {
                min: "0x200000".to_string(),
                max: "0x3fffff".to_string(),
                address: "1CfZWK1QTQE3eS9qn61dQjV89KDjZzfNcv".to_string(),
                status: 1,
            },
        ),
        (
            23,
            Wallet {
                min: "0x400000".to_string(),
                max: "0x7fffff".to_string(),
                address: "1L2GM8eE7mJWLdo3HZS6su1832NX2txaac".to_string(),
                status: 1,
            },
        ),
        (
            24,
            Wallet {
                min: "0x800000".to_string(),
                max: "0xffffff".to_string(),
                address: "1rSnXMr63jdCuegJFuidJqWxUPV7AtUf7".to_string(),
                status: 1,
            },
        ),
        (
            25,
            Wallet {
                min: "0x1000000".to_string(),
                max: "0x1ffffff".to_string(),
                address: "15JhYXn6Mx3oF4Y7PcTAv2wVVAuCFFQNiP".to_string(),
                status: 1,
            },
        ),
        (
            26,
            Wallet {
                min: "0x2000000".to_string(),
                max: "0x3ffffff".to_string(),
                address: "1JVnST957hGztonaWK6FougdtjxzHzRMMg".to_string(),
                status: 1,
            },
        ),
        (
            27,
            Wallet {
                min: "0x4000000".to_string(),
                max: "0x7ffffff".to_string(),
                address: "128z5d7nN7PkCuX5qoA4Ys6pmxUYnEy86k".to_string(),
                status: 1,
            },
        ),
        (
            28,
            Wallet {
                min: "0x8000000".to_string(),
                max: "0xfffffff".to_string(),
                address: "12jbtzBb54r97TCwW3G1gCFoumpckRAPdY".to_string(),
                status: 1,
            },
        ),
        (
            29,
            Wallet {
                min: "0x10000000".to_string(),
                max: "0x1fffffff".to_string(),
                address: "19EEC52krRUK1RkUAEZmQdjTyHT7Gp1TYT".to_string(),
                status: 1,
            },
        ),
        (
            30,
            Wallet {
                min: "0x20000000".to_string(),
                max: "0x3fffffff".to_string(),
                address: "1LHtnpd8nU5VHEMkG2TMYYNUjjLc992bps".to_string(),
                status: 1,
            },
        ),
        (
            31,
            Wallet {
                min: "0x40000000".to_string(),
                max: "0x7fffffff".to_string(),
                address: "1LhE6sCTuGae42Axu1L1ZB7L96yi9irEBE".to_string(),
                status: 1,
            },
        ),
        (
            32,
            Wallet {
                min: "0x80000000".to_string(),
                max: "0xffffffff".to_string(),
                address: "1FRoHA9xewq7DjrZ1psWJVeTer8gHRqEvR".to_string(),
                status: 1,
            },
        ),
        (
            33,
            Wallet {
                min: "0x100000000".to_string(),
                max: "0x1ffffffff".to_string(),
                address: "187swFMjz1G54ycVU56B7jZFHFTNVQFDiu".to_string(),
                status: 1,
            },
        ),
        (
            34,
            Wallet {
                min: "0x200000000".to_string(),
                max: "0x3ffffffff".to_string(),
                address: "1PWABE7oUahG2AFFQhhvViQovnCr4rEv7Q".to_string(),
                status: 1,
            },
        ),
        (
            35,
            Wallet {
                min: "0x400000000".to_string(),
                max: "0x7ffffffff".to_string(),
                address: "1PWCx5fovoEaoBowAvF5k91m2Xat9bMgwb".to_string(),
                status: 1,
            },
        ),
        (
            36,
            Wallet {
                min: "0x800000000".to_string(),
                max: "0xfffffffff".to_string(),
                address: "1Be2UF9NLfyLFbtm3TCbmuocc9N1Kduci1".to_string(),
                status: 1,
            },
        ),
        (
            37,
            Wallet {
                min: "0x1000000000".to_string(),
                max: "0x1fffffffff".to_string(),
                address: "14iXhn8bGajVWegZHJ18vJLHhntcpL4dex".to_string(),
                status: 1,
            },
        ),
        (
            38,
            Wallet {
                min: "0x2000000000".to_string(),
                max: "0x3fffffffff".to_string(),
                address: "1HBtApAFA9B2YZw3G2YKSMCtb3dVnjuNe2".to_string(),
                status: 1,
            },
        ),
        (
            39,
            Wallet {
                min: "0x4000000000".to_string(),
                max: "0x7fffffffff".to_string(),
                address: "122AJhKLEfkFBaGAd84pLp1kfE7xK3GdT8".to_string(),
                status: 1,
            },
        ),
        (
            40,
            Wallet {
                min: "0x8000000000".to_string(),
                max: "0xffffffffff".to_string(),
                address: "1EeAxcprB2PpCnr34VfZdFrkUWuxyiNEFv".to_string(),
                status: 1,
            },
        ),
        (
            41,
            Wallet {
                min: "0x10000000000".to_string(),
                max: "0x1ffffffffff".to_string(),
                address: "1L5sU9qvJeuwQUdt4y1eiLmquFxKjtHr3E".to_string(),
                status: 1,
            },
        ),
        (
            42,
            Wallet {
                min: "0x20000000000".to_string(),
                max: "0x3ffffffffff".to_string(),
                address: "1E32GPWgDyeyQac4aJxm9HVoLrrEYPnM4N".to_string(),
                status: 1,
            },
        ),
        (
            43,
            Wallet {
                min: "0x40000000000".to_string(),
                max: "0x7ffffffffff".to_string(),
                address: "1PiFuqGpG8yGM5v6rNHWS3TjsG6awgEGA1".to_string(),
                status: 1,
            },
        ),
        (
            44,
            Wallet {
                min: "0x80000000000".to_string(),
                max: "0xfffffffffff".to_string(),
                address: "1CkR2uS7LmFwc3T2jV8C1BhWb5mQaoxedF".to_string(),
                status: 1,
            },
        ),
        (
            45,
            Wallet {
                min: "0x100000000000".to_string(),
                max: "0x1fffffffffff".to_string(),
                address: "1NtiLNGegHWE3Mp9g2JPkgx6wUg4TW7bbk".to_string(),
                status: 1,
            },
        ),
        (
            46,
            Wallet {
                min: "0x200000000000".to_string(),
                max: "0x3fffffffffff".to_string(),
                address: "1F3JRMWudBaj48EhwcHDdpeuy2jwACNxjP".to_string(),
                status: 1,
            },
        ),
        (
            47,
            Wallet {
                min: "0x400000000000".to_string(),
                max: "0x7fffffffffff".to_string(),
                address: "1Pd8VvT49sHKsmqrQiP61RsVwmXCZ6ay7Z".to_string(),
                status: 1,
            },
        ),
        (
            48,
            Wallet {
                min: "0x800000000000".to_string(),
                max: "0xffffffffffff".to_string(),
                address: "1DFYhaB2J9q1LLZJWKTnscPWos9VBqDHzv".to_string(),
                status: 1,
            },
        ),
        (
            49,
            Wallet {
                min: "0x1000000000000".to_string(),
                max: "0x1ffffffffffff".to_string(),
                address: "12CiUhYVTTH33w3SPUBqcpMoqnApAV4WCF".to_string(),
                status: 1,
            },
        ),
        (
            50,
            Wallet {
                min: "0x2000000000000".to_string(),
                max: "0x3ffffffffffff".to_string(),
                address: "1MEzite4ReNuWaL5Ds17ePKt2dCxWEofwk".to_string(),
                status: 1,
            },
        ),
        (
            51,
            Wallet {
                min: "0x4000000000000".to_string(),
                max: "0x7ffffffffffff".to_string(),
                address: "1NpnQyZ7x24ud82b7WiRNvPm6N8bqGQnaS".to_string(),
                status: 1,
            },
        ),
        (
            52,
            Wallet {
                min: "0x8000000000000".to_string(),
                max: "0xfffffffffffff".to_string(),
                address: "15z9c9sVpu6fwNiK7dMAFgMYSK4GqsGZim".to_string(),
                status: 1,
            },
        ),
        (
            53,
            Wallet {
                min: "0x10000000000000".to_string(),
                max: "0x1fffffffffffff".to_string(),
                address: "15K1YKJMiJ4fpesTVUcByoz334rHmknxmT".to_string(),
                status: 1,
            },
        ),
        (
            54,
            Wallet {
                min: "0x20000000000000".to_string(),
                max: "0x3fffffffffffff".to_string(),
                address: "1KYUv7nSvXx4642TKeuC2SNdTk326uUpFy".to_string(),
                status: 1,
            },
        ),
        (
            55,
            Wallet {
                min: "0x40000000000000".to_string(),
                max: "0x7fffffffffffff".to_string(),
                address: "1LzhS3k3e9Ub8i2W1V8xQFdB8n2MYCHPCa".to_string(),
                status: 1,
            },
        ),
        (
            56,
            Wallet {
                min: "0x80000000000000".to_string(),
                max: "0xffffffffffffff".to_string(),
                address: "17aPYR1m6pVAacXg1PTDDU7XafvK1dxvhi".to_string(),
                status: 1,
            },
        ),
        (
            57,
            Wallet {
                min: "0x100000000000000".to_string(),
                max: "0x1ffffffffffffff".to_string(),
                address: "15c9mPGLku1HuW9LRtBf4jcHVpBUt8txKz".to_string(),
                status: 1,
            },
        ),
        (
            58,
            Wallet {
                min: "0x200000000000000".to_string(),
                max: "0x3ffffffffffffff".to_string(),
                address: "1Dn8NF8qDyyfHMktmuoQLGyjWmZXgvosXf".to_string(),
                status: 1,
            },
        ),
        (
            59,
            Wallet {
                min: "0x400000000000000".to_string(),
                max: "0x7ffffffffffffff".to_string(),
                address: "1HAX2n9Uruu9YDt4cqRgYcvtGvZj1rbUyt".to_string(),
                status: 1,
            },
        ),
        (
            60,
            Wallet {
                min: "0x800000000000000".to_string(),
                max: "0xfffffffffffffff".to_string(),
                address: "1Kn5h2qpgw9mWE5jKpk8PP4qvvJ1QVy8su".to_string(),
                status: 1,
            },
        ),
        (
            61,
            Wallet {
                min: "0x1000000000000000".to_string(),
                max: "0x1fffffffffffffff".to_string(),
                address: "1AVJKwzs9AskraJLGHAZPiaZcrpDr1U6AB".to_string(),
                status: 1,
            },
        ),
        (
            62,
            Wallet {
                min: "0x2000000000000000".to_string(),
                max: "0x3fffffffffffffff".to_string(),
                address: "1Me6EfpwZK5kQziBwBfvLiHjaPGxCKLoJi".to_string(),
                status: 1,
            },
        ),
        (
            63,
            Wallet {
                min: "0x4000000000000000".to_string(),
                max: "0x7fffffffffffffff".to_string(),
                address: "1NpYjtLira16LfGbGwZJ5JbDPh3ai9bjf4".to_string(),
                status: 1,
            },
        ),
        (
            64,
            Wallet {
                min: "0x8000000000000000".to_string(),
                max: "0xffffffffffffffff".to_string(),
                address: "16jY7qLJnxb7CHZyqBP8qca9d51gAjyXQN".to_string(),
                status: 1,
            },
        ),
        (
            65,
            Wallet {
                min: "0x10000000000000000".to_string(),
                max: "0x1ffffffffffffffff".to_string(),
                address: "18ZMbwUFLMHoZBbfpCjUJQTCMCbktshgpe".to_string(),
                status: 1,
            },
        ),
        (
            66,
            Wallet {
                min: "0x20000000000000000".to_string(),
                max: "0x3ffffffffffffffff".to_string(),
                address: "13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so".to_string(),
                status: 1,
            },
        ),
        (
            67,
            Wallet {
                min: "0x40000000000000000".to_string(),
                max: "0x7ffffffffffffffff".to_string(),
                address: "1BY8GQbnueYofwSuFAT3USAhGjPrkxDdW9".to_string(),
                status: 1,
            },
        ),
        (
            68,
            Wallet {
                min: "0x80000000000000000".to_string(),
                max: "0xfffffffffffffffff".to_string(),
                address: "1MVDYgVaSN6iKKEsbzRUAYFrYJadLYZvvZ".to_string(),
                status: 1,
            },
        ),
        (
            69,
            Wallet {
                min: "0x100000000000000000".to_string(),
                max: "0x1fffffffffffffffff".to_string(),
                address: "19vkiEajfhuZ8bs8Zu2jgmC6oqZbWqhxhG".to_string(),
                status: 1,
            },
        ),
        (
            70,
            Wallet {
                min: "0x200000000000000000".to_string(),
                max: "0x3fffffffffffffffff".to_string(),
                address: "19YZECXj3SxEZMoUeJ1yiPsw8xANe7M7QR".to_string(),
                status: 1,
            },
        ),
        (
            71,
            Wallet {
                min: "0x400000000000000000".to_string(),
                max: "0x7fffffffffffffffff".to_string(),
                address: "1PWo3JeB9jrGwfHDNpdGK54CRas7fsVzXU".to_string(),
                status: 1,
            },
        ),
        (
            72,
            Wallet {
                min: "0x800000000000000000".to_string(),
                max: "0xffffffffffffffffff".to_string(),
                address: "1JTK7s9YVYywfm5XUH7RNhHJH1LshCaRFR".to_string(),
                status: 1,
            },
        ),
        (
            73,
            Wallet {
                min: "0x1000000000000000000".to_string(),
                max: "0x1ffffffffffffffffff".to_string(),
                address: "12VVRNPi4SJqUTsp6FmqDqY5sGosDtysn4".to_string(),
                status: 1,
            },
        ),
        (
            74,
            Wallet {
                min: "0x2000000000000000000".to_string(),
                max: "0x3ffffffffffffffffff".to_string(),
                address: "1FWGcVDK3JGzCC3WtkYetULPszMaK2Jksv".to_string(),
                status: 1,
            },
        ),
        (
            75,
            Wallet {
                min: "0x4000000000000000000".to_string(),
                max: "0x7ffffffffffffffffff".to_string(),
                address: "1J36UjUByGroXcCvmj13U6uwaVv9caEeAt".to_string(),
                status: 1,
            },
        ),
        (
            76,
            Wallet {
                min: "0x8000000000000000000".to_string(),
                max: "0xfffffffffffffffffff".to_string(),
                address: "1DJh2eHFYQfACPmrvpyWc8MSTYKh7w9eRF".to_string(),
                status: 1,
            },
        ),
        (
            77,
            Wallet {
                min: "0x10000000000000000000".to_string(),
                max: "0x1fffffffffffffffffff".to_string(),
                address: "1Bxk4CQdqL9p22JEtDfdXMsng1XacifUtE".to_string(),
                status: 1,
            },
        ),
        (
            78,
            Wallet {
                min: "0x20000000000000000000".to_string(),
                max: "0x3fffffffffffffffffff".to_string(),
                address: "15qF6X51huDjqTmF9BJgxXdt1xcj46Jmhb".to_string(),
                status: 1,
            },
        ),
        (
            79,
            Wallet {
                min: "0x40000000000000000000".to_string(),
                max: "0x7fffffffffffffffffff".to_string(),
                address: "1ARk8HWJMn8js8tQmGUJeQHjSE7KRkn2t8".to_string(),
                status: 1,
            },
        ),
        (
            80,
            Wallet {
                min: "0x80000000000000000000".to_string(),
                max: "0xffffffffffffffffffff".to_string(),
                address: "1BCf6rHUW6m3iH2ptsvnjgLruAiPQQepLe".to_string(),
                status: 1,
            },
        ),
        (
            81,
            Wallet {
                min: "0x100000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffff".to_string(),
                address: "15qsCm78whspNQFydGJQk5rexzxTQopnHZ".to_string(),
                status: 1,
            },
        ),
        (
            82,
            Wallet {
                min: "0x200000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffff".to_string(),
                address: "13zYrYhhJxp6Ui1VV7pqa5WDhNWM45ARAC".to_string(),
                status: 1,
            },
        ),
        (
            83,
            Wallet {
                min: "0x400000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffff".to_string(),
                address: "14MdEb4eFcT3MVG5sPFG4jGLuHJSnt1Dk2".to_string(),
                status: 1,
            },
        ),
        (
            84,
            Wallet {
                min: "0x800000000000000000000".to_string(),
                max: "0xfffffffffffffffffffff".to_string(),
                address: "1CMq3SvFcVEcpLMuuH8PUcNiqsK1oicG2D".to_string(),
                status: 1,
            },
        ),
        (
            85,
            Wallet {
                min: "0x1000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffff".to_string(),
                address: "1Kh22PvXERd2xpTQk3ur6pPEqFeckCJfAr".to_string(),
                status: 1,
            },
        ),
        (
            86,
            Wallet {
                min: "0x2000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffff".to_string(),
                address: "1K3x5L6G57Y494fDqBfrojD28UJv4s5JcK".to_string(),
                status: 1,
            },
        ),
        (
            87,
            Wallet {
                min: "0x4000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffff".to_string(),
                address: "1PxH3K1Shdjb7gSEoTX7UPDZ6SH4qGPrvq".to_string(),
                status: 1,
            },
        ),
        (
            88,
            Wallet {
                min: "0x8000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffff".to_string(),
                address: "16AbnZjZZipwHMkYKBSfswGWKDmXHjEpSf".to_string(),
                status: 1,
            },
        ),
        (
            89,
            Wallet {
                min: "0x10000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffff".to_string(),
                address: "19QciEHbGVNY4hrhfKXmcBBCrJSBZ6TaVt".to_string(),
                status: 1,
            },
        ),
        (
            90,
            Wallet {
                min: "0x20000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffff".to_string(),
                address: "1L12FHH2FHjvTviyanuiFVfmzCy46RRATU".to_string(),
                status: 1,
            },
        ),
        (
            91,
            Wallet {
                min: "0x40000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffff".to_string(),
                address: "1EzVHtmbN4fs4MiNk3ppEnKKhsmXYJ4s74".to_string(),
                status: 1,
            },
        ),
        (
            92,
            Wallet {
                min: "0x80000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffff".to_string(),
                address: "1AE8NzzgKE7Yhz7BWtAcAAxiFMbPo82NB5".to_string(),
                status: 1,
            },
        ),
        (
            93,
            Wallet {
                min: "0x100000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffff".to_string(),
                address: "17Q7tuG2JwFFU9rXVj3uZqRtioH3mx2Jad".to_string(),
                status: 1,
            },
        ),
        (
            94,
            Wallet {
                min: "0x200000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffff".to_string(),
                address: "1K6xGMUbs6ZTXBnhw1pippqwK6wjBWtNpL".to_string(),
                status: 1,
            },
        ),
        (
            95,
            Wallet {
                min: "0x400000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffff".to_string(),
                address: "19eVSDuizydXxhohGh8Ki9WY9KsHdSwoQC".to_string(),
                status: 1,
            },
        ),
        (
            96,
            Wallet {
                min: "0x800000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffff".to_string(),
                address: "15ANYzzCp5BFHcCnVFzXqyibpzgPLWaD8b".to_string(),
                status: 1,
            },
        ),
        (
            97,
            Wallet {
                min: "0x1000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffff".to_string(),
                address: "18ywPwj39nGjqBrQJSzZVq2izR12MDpDr8".to_string(),
                status: 1,
            },
        ),
        (
            98,
            Wallet {
                min: "0x2000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffff".to_string(),
                address: "1CaBVPrwUxbQYYswu32w7Mj4HR4maNoJSX".to_string(),
                status: 1,
            },
        ),
        (
            99,
            Wallet {
                min: "0x4000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffff".to_string(),
                address: "1JWnE6p6UN7ZJBN7TtcbNDoRcjFtuDWoNL".to_string(),
                status: 1,
            },
        ),
        (
            100,
            Wallet {
                min: "0x8000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffff".to_string(),
                address: "1KCgMv8fo2TPBpddVi9jqmMmcne9uSNJ5F".to_string(),
                status: 1,
            },
        ),
        (
            101,
            Wallet {
                min: "0x10000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffff".to_string(),
                address: "1CKCVdbDJasYmhswB6HKZHEAnNaDpK7W4n".to_string(),
                status: 1,
            },
        ),
        (
            102,
            Wallet {
                min: "0x20000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffff".to_string(),
                address: "1PXv28YxmYMaB8zxrKeZBW8dt2HK7RkRPX".to_string(),
                status: 1,
            },
        ),
        (
            103,
            Wallet {
                min: "0x40000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffff".to_string(),
                address: "1AcAmB6jmtU6AiEcXkmiNE9TNVPsj9DULf".to_string(),
                status: 1,
            },
        ),
        (
            104,
            Wallet {
                min: "0x80000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffff".to_string(),
                address: "1EQJvpsmhazYCcKX5Au6AZmZKRnzarMVZu".to_string(),
                status: 1,
            },
        ),
        (
            105,
            Wallet {
                min: "0x100000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffff".to_string(),
                address: "1CMjscKB3QW7SDyQ4c3C3DEUHiHRhiZVib".to_string(),
                status: 1,
            },
        ),
        (
            106,
            Wallet {
                min: "0x200000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffffff".to_string(),
                address: "18KsfuHuzQaBTNLASyj15hy4LuqPUo1FNB".to_string(),
                status: 1,
            },
        ),
        (
            107,
            Wallet {
                min: "0x400000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffffff".to_string(),
                address: "15EJFC5ZTs9nhsdvSUeBXjLAuYq3SWaxTc".to_string(),
                status: 1,
            },
        ),
        (
            108,
            Wallet {
                min: "0x800000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffffff".to_string(),
                address: "1HB1iKUqeffnVsvQsbpC6dNi1XKbyNuqao".to_string(),
                status: 1,
            },
        ),
        (
            109,
            Wallet {
                min: "0x1000000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffffff".to_string(),
                address: "1GvgAXVCbA8FBjXfWiAms4ytFeJcKsoyhL".to_string(),
                status: 1,
            },
        ),
        (
            110,
            Wallet {
                min: "0x2000000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffffff".to_string(),
                address: "12JzYkkN76xkwvcPT6AWKZtGX6w2LAgsJg".to_string(),
                status: 1,
            },
        ),
        (
            111,
            Wallet {
                min: "0x4000000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffffff".to_string(),
                address: "1824ZJQ7nKJ9QFTRBqn7z7dHV5EGpzUpH3".to_string(),
                status: 1,
            },
        ),
        (
            112,
            Wallet {
                min: "0x8000000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffffff".to_string(),
                address: "18A7NA9FTsnJxWgkoFfPAFbQzuQxpRtCos".to_string(),
                status: 1,
            },
        ),
        (
            113,
            Wallet {
                min: "0x10000000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffffff".to_string(),
                address: "1NeGn21dUDDeqFQ63xb2SpgUuXuBLA4WT4".to_string(),
                status: 1,
            },
        ),
        (
            114,
            Wallet {
                min: "0x20000000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffffffff".to_string(),
                address: "174SNxfqpdMGYy5YQcfLbSTK3MRNZEePoy".to_string(),
                status: 1,
            },
        ),
        (
            115,
            Wallet {
                min: "0x40000000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffffffff".to_string(),
                address: "1NLbHuJebVwUZ1XqDjsAyfTRUPwDQbemfv".to_string(),
                status: 1,
            },
        ),
        (
            116,
            Wallet {
                min: "0x80000000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffffffff".to_string(),
                address: "1MnJ6hdhvK37VLmqcdEwqC3iFxyWH2PHUV".to_string(),
                status: 1,
            },
        ),
        (
            117,
            Wallet {
                min: "0x100000000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffffffff".to_string(),
                address: "1KNRfGWw7Q9Rmwsc6NT5zsdvEb9M2Wkj5Z".to_string(),
                status: 1,
            },
        ),
        (
            118,
            Wallet {
                min: "0x200000000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffffffff".to_string(),
                address: "1PJZPzvGX19a7twf5HyD2VvNiPdHLzm9F6".to_string(),
                status: 1,
            },
        ),
        (
            119,
            Wallet {
                min: "0x400000000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffffffff".to_string(),
                address: "1GuBBhf61rnvRe4K8zu8vdQB3kHzwFqSy7".to_string(),
                status: 1,
            },
        ),
        (
            120,
            Wallet {
                min: "0x800000000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffffffff".to_string(),
                address: "17s2b9ksz5y7abUm92cHwG8jEPCzK3dLnT".to_string(),
                status: 1,
            },
        ),
        (
            121,
            Wallet {
                min: "0x1000000000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffffffff".to_string(),
                address: "1GDSuiThEV64c166LUFC9uDcVdGjqkxKyh".to_string(),
                status: 1,
            },
        ),
        (
            122,
            Wallet {
                min: "0x2000000000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffffffffff".to_string(),
                address: "1Me3ASYt5JCTAK2XaC32RMeH34PdprrfDx".to_string(),
                status: 1,
            },
        ),
        (
            123,
            Wallet {
                min: "0x4000000000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffffffffff".to_string(),
                address: "1CdufMQL892A69KXgv6UNBD17ywWqYpKut".to_string(),
                status: 1,
            },
        ),
        (
            124,
            Wallet {
                min: "0x8000000000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffffffffff".to_string(),
                address: "1BkkGsX9ZM6iwL3zbqs7HWBV7SvosR6m8N".to_string(),
                status: 1,
            },
        ),
        (
            125,
            Wallet {
                min: "0x10000000000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffffffffff".to_string(),
                address: "1PXAyUB8ZoH3WD8n5zoAthYjN15yN5CVq5".to_string(),
                status: 1,
            },
        ),
        (
            126,
            Wallet {
                min: "0x20000000000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffffffffff".to_string(),
                address: "1AWCLZAjKbV1P7AHvaPNCKiB7ZWVDMxFiz".to_string(),
                status: 1,
            },
        ),
        (
            127,
            Wallet {
                min: "0x40000000000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffffffffff".to_string(),
                address: "1G6EFyBRU86sThN3SSt3GrHu1sA7w7nzi4".to_string(),
                status: 1,
            },
        ),
        (
            128,
            Wallet {
                min: "0x80000000000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffffffffff".to_string(),
                address: "1MZ2L1gFrCtkkn6DnTT2e4PFUTHw9gNwaj".to_string(),
                status: 1,
            },
        ),
        (
            129,
            Wallet {
                min: "0x100000000000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffffffffff".to_string(),
                address: "1Hz3uv3nNZzBVMXLGadCucgjiCs5W9vaGz".to_string(),
                status: 1,
            },
        ),
        (
            130,
            Wallet {
                min: "0x200000000000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffffffffffff".to_string(),
                address: "1Fo65aKq8s8iquMt6weF1rku1moWVEd5Ua".to_string(),
                status: 1,
            },
        ),
        (
            131,
            Wallet {
                min: "0x400000000000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffffffffffff".to_string(),
                address: "16zRPnT8znwq42q7XeMkZUhb1bKqgRogyy".to_string(),
                status: 1,
            },
        ),
        (
            132,
            Wallet {
                min: "0x800000000000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffffffffffff".to_string(),
                address: "1KrU4dHE5WrW8rhWDsTRjR21r8t3dsrS3R".to_string(),
                status: 1,
            },
        ),
        (
            133,
            Wallet {
                min: "0x1000000000000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffffffffffff".to_string(),
                address: "17uDfp5r4n441xkgLFmhNoSW1KWp6xVLD".to_string(),
                status: 1,
            },
        ),
        (
            134,
            Wallet {
                min: "0x2000000000000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffffffffffff".to_string(),
                address: "13A3JrvXmvg5w9XGvyyR4JEJqiLz8ZySY3".to_string(),
                status: 1,
            },
        ),
        (
            135,
            Wallet {
                min: "0x4000000000000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffffffffffff".to_string(),
                address: "16RGFo6hjq9ym6Pj7N5H7L1NR1rVPJyw2v".to_string(),
                status: 1,
            },
        ),
        (
            136,
            Wallet {
                min: "0x8000000000000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffffffffffff".to_string(),
                address: "1UDHPdovvR985NrWSkdWQDEQ1xuRiTALq".to_string(),
                status: 1,
            },
        ),
        (
            137,
            Wallet {
                min: "0x10000000000000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffffffffffff".to_string(),
                address: "15nf31J46iLuK1ZkTnqHo7WgN5cARFK3RA".to_string(),
                status: 1,
            },
        ),
        (
            138,
            Wallet {
                min: "0x20000000000000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffffffffffffff".to_string(),
                address: "1Ab4vzG6wEQBDNQM1B2bvUz4fqXXdFk2WT".to_string(),
                status: 1,
            },
        ),
        (
            139,
            Wallet {
                min: "0x40000000000000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffffffffffffff".to_string(),
                address: "1Fz63c775VV9fNyj25d9Xfw3YHE6sKCxbt".to_string(),
                status: 1,
            },
        ),
        (
            140,
            Wallet {
                min: "0x80000000000000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffffffffffffff".to_string(),
                address: "1QKBaU6WAeycb3DbKbLBkX7vJiaS8r42Xo".to_string(),
                status: 1,
            },
        ),
        (
            141,
            Wallet {
                min: "0x100000000000000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffffffffffffff".to_string(),
                address: "1CD91Vm97mLQvXhrnoMChhJx4TP9MaQkJo".to_string(),
                status: 1,
            },
        ),
        (
            142,
            Wallet {
                min: "0x200000000000000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffffffffffffff".to_string(),
                address: "15MnK2jXPqTMURX4xC3h4mAZxyCcaWWEDD".to_string(),
                status: 1,
            },
        ),
        (
            143,
            Wallet {
                min: "0x400000000000000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffffffffffffff".to_string(),
                address: "13N66gCzWWHEZBxhVxG18P8wyjEWF9Yoi1".to_string(),
                status: 1,
            },
        ),
        (
            144,
            Wallet {
                min: "0x800000000000000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1NevxKDYuDcCh1ZMMi6ftmWwGrZKC6j7Ux".to_string(),
                status: 1,
            },
        ),
        (
            145,
            Wallet {
                min: "0x1000000000000000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffffffffffffff".to_string(),
                address: "19GpszRNUej5yYqxXoLnbZWKew3KdVLkXg".to_string(),
                status: 1,
            },
        ),
        (
            146,
            Wallet {
                min: "0x2000000000000000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1M7ipcdYHey2Y5RZM34MBbpugghmjaV89P".to_string(),
                status: 1,
            },
        ),
        (
            147,
            Wallet {
                min: "0x4000000000000000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffffffffffffffff".to_string(),
                address: "18aNhurEAJsw6BAgtANpexk5ob1aGTwSeL".to_string(),
                status: 1,
            },
        ),
        (
            148,
            Wallet {
                min: "0x8000000000000000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1FwZXt6EpRT7Fkndzv6K4b4DFoT4trbMrV".to_string(),
                status: 1,
            },
        ),
        (
            149,
            Wallet {
                min: "0x10000000000000000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1CXvTzR6qv8wJ7eprzUKeWxyGcHwDYP1i2".to_string(),
                status: 1,
            },
        ),
        (
            150,
            Wallet {
                min: "0x20000000000000000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1MUJSJYtGPVGkBCTqGspnxyHahpt5Te8jy".to_string(),
                status: 1,
            },
        ),
        (
            151,
            Wallet {
                min: "0x40000000000000000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffffffffffffffff".to_string(),
                address: "13Q84TNNvgcL3HJiqQPvyBb9m4hxjS3jkV".to_string(),
                status: 1,
            },
        ),
        (
            152,
            Wallet {
                min: "0x80000000000000000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1LuUHyrQr8PKSvbcY1v1PiuGuqFjWpDumN".to_string(),
                status: 1,
            },
        ),
        (
            153,
            Wallet {
                min: "0x100000000000000000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "18192XpzzdDi2K11QVHR7td2HcPS6Qs5vg".to_string(),
                status: 1,
            },
        ),
        (
            154,
            Wallet {
                min: "0x200000000000000000000000000000000000000".to_string(),
                max: "0x3ffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1NgVmsCCJaKLzGyKLFJfVequnFW9ZvnMLN".to_string(),
                status: 1,
            },
        ),
        (
            155,
            Wallet {
                min: "0x400000000000000000000000000000000000000".to_string(),
                max: "0x7ffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1AoeP37TmHdFh8uN72fu9AqgtLrUwcv2wJ".to_string(),
                status: 1,
            },
        ),
        (
            156,
            Wallet {
                min: "0x800000000000000000000000000000000000000".to_string(),
                max: "0xfffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1FTpAbQa4h8trvhQXjXnmNhqdiGBd1oraE".to_string(),
                status: 1,
            },
        ),
        (
            157,
            Wallet {
                min: "0x1000000000000000000000000000000000000000".to_string(),
                max: "0x1fffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "14JHoRAdmJg3XR4RjMDh6Wed6ft6hzbQe9".to_string(),
                status: 1,
            },
        ),
        (
            158,
            Wallet {
                min: "0x2000000000000000000000000000000000000000".to_string(),
                max: "0x3fffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "19z6waranEf8CcP8FqNgdwUe1QRxvUNKBG".to_string(),
                status: 1,
            },
        ),
        (
            159,
            Wallet {
                min: "0x4000000000000000000000000000000000000000".to_string(),
                max: "0x7fffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "14u4nA5sugaswb6SZgn5av2vuChdMnD9E5".to_string(),
                status: 1,
            },
        ),
        (
            160,
            Wallet {
                min: "0x8000000000000000000000000000000000000000".to_string(),
                max: "0xffffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "1NBC8uXJy1GiJ6drkiZa1WuKn51ps7EPTv".to_string(),
                status: 1,
            },
        ),
        (
            161,
            Wallet {
                min: "0x10000000000000000000000000000000000000000".to_string(),
                max: "0x1ffffffffffffffffffffffffffffffffffffffff".to_string(),
                address: "18bHfcm8kGoAhBaQXzzVcG5534mdpWK981".to_string(),
                status: 1,
            },
        ),
    ])
}
