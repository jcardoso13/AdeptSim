
fn get_register_label(reg: u8) -> &'static str{

    match reg {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",

        5 => "t0",
        6 => "t1",
        7 => "t2",

        8 => "s0/fp",
        9 => "s1",

        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",

        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",

        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",

        _ => panic!("Invalid Register access"),
    }
}

#[cfg(test)]
mod tests {
    ////////////////////////////////////////////////////////////////////////////////
    // Registers Printing Test
    ////////////////////////////////////////////////////////////////////////////////
    /// Test Registers Printing
    #[test]
    fn print_registers() {
        // 0 = zero
        assert_eq!("zero", super::get_register_label(0));
        // 1 = ra
        assert_eq!("ra", super::get_register_label(1));
        // 2 = sp
        assert_eq!("sp", super::get_register_label(2));
        // 3 = gp
        assert_eq!("gp", super::get_register_label(3));
        // 4 = tp
        assert_eq!("tp", super::get_register_label(4));
        // 5 = t0
        assert_eq!("t0", super::get_register_label(5));
        // 6 = t1
        assert_eq!("t1", super::get_register_label(6));
        // 7 = t2
        assert_eq!("t2", super::get_register_label(7));
        // 8 = s0/fp
        assert_eq!("s0/fp", super::get_register_label(8));
        // 9 = s1
        assert_eq!("s1", super::get_register_label(9));
        // 10 = a0
        assert_eq!("a0", super::get_register_label(10));
        // 11 = a1
        assert_eq!("a1", super::get_register_label(11));
        // 12 = a2
        assert_eq!("a2", super::get_register_label(12));
        // 13 = a3
        assert_eq!("a3", super::get_register_label(13));
        // 14 = a4
        assert_eq!("a4", super::get_register_label(14));
        // 15 = a5
        assert_eq!("a5", super::get_register_label(15));
        // 16 = a6
        assert_eq!("a6", super::get_register_label(16));
        // 17 = a7
        assert_eq!("a7", super::get_register_label(17));
        // 18 = s2
        assert_eq!("s2", super::get_register_label(18));
        // 19 = s3
        assert_eq!("s3", super::get_register_label(19));
        // 20 = s4
        assert_eq!("s4", super::get_register_label(20));
        // 21 = s5
        assert_eq!("s5", super::get_register_label(21));
        // 22 = s6
        assert_eq!("s6", super::get_register_label(22));
        // 23 = s7
        assert_eq!("s7", super::get_register_label(23));
        // 24 = s8
        assert_eq!("s8", super::get_register_label(24));
        // 25 = s9
        assert_eq!("s9", super::get_register_label(25));
        // 26 = s10
        assert_eq!("s10", super::get_register_label(26));
        // 27 = s11
        assert_eq!("s11", super::get_register_label(27));
        // 28 = t3
        assert_eq!("t3", super::get_register_label(28));
        // 29 = t4
        assert_eq!("t4", super::get_register_label(29));
        // 30 = t5
        assert_eq!("t5", super::get_register_label(30));
        // 31 = t6
        assert_eq!("t6", super::get_register_label(31));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Register Label Panic
    ////////////////////////////////////////////////////////////////////////////////
    /// Test Registers Printing
    #[test]
    #[should_panic]
    fn print_registers_panic() {
        super::get_register_label(35);
    }
}
