pub mod bcm;

pub match bcm {
    3 =>2, //SDA BCM2
    5 =>3, //SCL BCM3
    7 =>4, //GPCLK0 BCM4
    //ground
    11 =>17, //BCM17
    13 =>27,
    15 =>22,
    //3v3 power
    19 =>10,
    21 =>9,
    23 =>11,
    //ground
    27 =>0,
    29 =>5,
    31 =>6,
    33 =>13,
    35 =>19,
    37 =>26,
    //ground
}