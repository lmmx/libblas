use libblas::level1::complex;
use num_complex::Complex;
mod utils;

#[test]
fn axpy() {
    let x = vec![
        Complex::new(1.262_954_284_880_793_3, 0.763_593_461_140_459_6),
        Complex::new(-0.326_233_360_705_649_4, -0.799_009_248_989_368_2),
        Complex::new(1.329_799_262_922_500_6, -1.147_657_009_236_351_4),
        Complex::new(1.272_429_321_429_404_7, -0.28946157368822334),
        Complex::new(0.414_641_434_456_408_2, -0.29921511789731614),
        Complex::new(-1.539_950_041_903_709_5, -0.411_510_832_795_067),
        Complex::new(-0.928_567_034_713_538_1, 0.252_223_448_156_132_3),
        Complex::new(-0.294_720_446_790_560_2, -0.891_921_127_284_568_6),
        Complex::new(-0.005_767_172_747_536_955, 0.43568329935571865),
        Complex::new(2.404_653_388_857_951, -1.237_538_421_929_958),
    ];
    let mut y = vec![
        Complex::new(-0.560_475_646_552_212_6, 1.224_081_797_439_461_5),
        Complex::new(-0.230_177_489_483_279_96, 0.359_813_827_057_363_8),
        Complex::new(1.558_708_314_149_124, 0.40077145059405217),
        Complex::new(0.070_508_391_424_576, 0.110_682_715_945_119_7),
        Complex::new(0.129_287_735_160_946_24, -0.555_841_134_754_074_9),
        Complex::new(1.715_064_986_883_281, 1.786_913_136_803_078_2),
        Complex::new(0.460_916_205_989_202_3, 0.497_850_478_229_239_4),
        Complex::new(-1.265_061_234_606_534, -1.966_617_156_629_638_2),
        Complex::new(-0.686_852_851_893_526_1, 0.701_355_901_563_685_5),
        Complex::new(-0.445_661_970_099_958_06, -0.47279140772793404),
    ];
    let a = Complex::new(23.0, 32.0);
    let expect = vec![
        Complex::new(4.052482149211329, 59.201_268_519_855_41),
        Complex::new(17.834_751_181_946_57, -28.456_866_442_278_89),
        Complex::new(68.869_115_656_929_89, 16.558_236_651_677_99),
        Complex::new(38.599_153_142_324_03, 34.170_804_806_856_93),
        Complex::new(19.240924500372454, 5.830_737_056_212_717),
        Complex::new(-20.535439327459894, -56.956_237_358_402_16),
        Complex::new(-28.967275933418406, -23.415_155_325_012_936),
        Complex::new(20.497_844_562_316_78, -31.911_857_381_472_643),
        Complex::new(-14.761363404469874, 10.537_522_258_824_032),
        Complex::new(94.462_595_475_391_57, 48.012_733_331_337_46),
    ];

    complex::axpy(10, &a, &x, 1, &mut y, 1);
    assert_eq!(y, expect);

    // let x = vec![
    //         Complex::new(1.2629542848807933098,0.76359346114045956),
    //         Complex::new(-0.3262333607056494000, -0.79900924898936820),
    //         Complex::new(1.3297992629225006134, -1.14765700923635139),
    //         Complex::new(1.2724293214294046805, -0.28946157368822334),
    //         Complex::new(0.4146414344564082199, -0.29921511789731614),
    //         Complex::new(-1.5399500419037095433, -0.41151083279506701),
    //         Complex::new(-0.9285670347135380753,  0.25222344815613229),
    //         Complex::new(-0.2947204467905601977, -0.89192112728456863),
    //         Complex::new(-0.0057671727475369552, 0.43568329935571865),
    //         Complex::new(2.4046533888579508798, -1.23753842192995811)
    //     ],
    // };
    let mut y = vec![
        Complex::new(-0.560_475_646_552_212_6, 1.224_081_797_439_461_5),
        Complex::new(-0.230_177_489_483_279_96, 0.359_813_827_057_363_8),
        Complex::new(1.558_708_314_149_124, 0.40077145059405217),
        Complex::new(0.070_508_391_424_576, 0.110_682_715_945_119_7),
        Complex::new(0.129_287_735_160_946_24, -0.555_841_134_754_074_9),
        Complex::new(1.715_064_986_883_281, 1.786_913_136_803_078_2),
        Complex::new(0.460_916_205_989_202_3, 0.497_850_478_229_239_4),
        Complex::new(-1.265_061_234_606_534, -1.966_617_156_629_638_2),
        Complex::new(-0.686_852_851_893_526_1, 0.701_355_901_563_685_5),
        Complex::new(-0.445_661_970_099_958_06, -0.47279140772793404),
    ];
    let expect = vec![
        Complex::new(4.052482149211329, 59.201_268_519_855_41),
        Complex::new(-0.230_177_489_483_279_96, 0.359_813_827_057_363_8),
        Complex::new(68.869_115_656_929_89, 16.558_236_651_677_99),
        Complex::new(0.070_508_391_424_576, 0.110_682_715_945_119_7),
        Complex::new(19.240924500372454, 5.830_737_056_212_717),
        Complex::new(1.715_064_986_883_281, 1.786_913_136_803_078_2),
        Complex::new(-28.967275933418406, -23.415_155_325_012_936),
        Complex::new(-1.265_061_234_606_534, -1.966_617_156_629_638_2),
        Complex::new(-14.761363404469874, 10.537_522_258_824_032),
        Complex::new(-0.445_661_970_099_958_06, -0.47279140772793404),
    ];

    complex::axpy(5, &a, &x, 2, &mut y, 2);
    assert_eq!(y, expect);

    let x = vec![Complex::new(3.0, 4.0)];
    let mut y = vec![Complex::new(1.0, 2.0)];
    let a = Complex::new(0.0, 0.0);
    complex::axpy(1, &a, &x, 1, &mut y, 1);
    assert_eq!(y, vec![Complex::new(1.0, 2.0)]);

    let x = vec![Complex::new(9.0, 4.0)];
    let mut y = vec![Complex::new(3.0, 4.0)];
    let a = Complex::new(1.0, 2.0);
    complex::axpy(0, &a, &x, 1, &mut y, 1);
    assert_eq!(y, vec![Complex::new(3.0, 4.0)]);

    let x = vec![
        Complex::new(1.262_954_284_880_793_3, 0.763_593_461_140_459_6),
        Complex::new(-0.326_233_360_705_649_4, -0.799_009_248_989_368_2),
        Complex::new(1.329_799_262_922_500_6, -1.147_657_009_236_351_4),
        Complex::new(1.272_429_321_429_404_7, -0.28946157368822334),
        Complex::new(0.414_641_434_456_408_2, -0.29921511789731614),
        Complex::new(-1.539_950_041_903_709_5, -0.411_510_832_795_067),
        Complex::new(-0.928_567_034_713_538_1, 0.252_223_448_156_132_3),
        Complex::new(-0.294_720_446_790_560_2, -0.891_921_127_284_568_6),
        Complex::new(-0.005_767_172_747_536_955, 0.43568329935571865),
        Complex::new(2.404_653_388_857_951, -1.237_538_421_929_958),
    ];
    let mut y = vec![
        Complex::new(-0.560_475_646_552_212_6, 1.224_081_797_439_461_5),
        Complex::new(-0.230_177_489_483_279_96, 0.359_813_827_057_363_8),
        Complex::new(1.558_708_314_149_124, 0.40077145059405217),
        Complex::new(0.070_508_391_424_576, 0.110_682_715_945_119_7),
        Complex::new(0.129_287_735_160_946_24, -0.555_841_134_754_074_9),
        Complex::new(1.715_064_986_883_281, 1.786_913_136_803_078_2),
        Complex::new(0.460_916_205_989_202_3, 0.497_850_478_229_239_4),
        Complex::new(-1.265_061_234_606_534, -1.966_617_156_629_638_2),
        Complex::new(-0.686_852_851_893_526_1, 0.701_355_901_563_685_5),
        Complex::new(-0.445_661_970_099_958_06, -0.47279140772793404),
    ];

    let a = Complex::new(23.0, 32.0);

    let expect = vec![
        Complex::new(4.052482149211329, 59.201_268_519_855_41),
        Complex::new(-0.230_177_489_483_279_96, 0.359_813_827_057_363_8),
        Complex::new(68.869_115_656_929_89, 16.558_236_651_677_99),
        Complex::new(0.070_508_391_424_576, 0.110_682_715_945_119_7),
        Complex::new(19.240924500372454, 5.830_737_056_212_717),
        Complex::new(1.715_064_986_883_281, 1.786_913_136_803_078_2),
        Complex::new(-28.967275933418406, -23.415_155_325_012_936),
        Complex::new(-1.265_061_234_606_534, -1.966_617_156_629_638_2),
        Complex::new(-14.761363404469874, 10.537_522_258_824_032),
        Complex::new(-0.445_661_970_099_958_06, -0.47279140772793404),
    ];

    complex::axpy(5, &a, &x, -2, &mut y, -2);
    assert_eq!(y, expect);
}

#[test]
fn copy() {
    let x = vec![
        Complex::new(1.0, 5.0),
        Complex::new(2.0, 6.0),
        Complex::new(3.0, 7.0),
        Complex::new(4.0, 8.0),
    ];
    let mut y = vec![
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];
    let expect = vec![
        Complex::new(1.0, 5.0),
        Complex::new(2.0, 6.0),
        Complex::new(3.0, 7.0),
        Complex::new(4.0, 8.0),
    ];
    complex::copy(4, &x, 1, &mut y, 1);
    assert_eq!(y, expect);

    let mut y = vec![
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];
    complex::copy(4, &x, -1, &mut y, -1);
    assert_eq!(y, expect);

    let mut y = vec![
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];
    let expect = vec![
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];
    complex::copy(0, &x, -1, &mut y, -1);
    assert_eq!(y, expect);
}

#[test]
fn dotc() {
    let x = vec![
        Complex::new(1.0, 5.0),
        Complex::new(2.0, 6.0),
        Complex::new(3.0, 7.0),
        Complex::new(4.0, 8.0),
    ];
    let y = vec![
        Complex::new(5.0, 9.0),
        Complex::new(6.0, 10.0),
        Complex::new(7.0, 11.0),
        Complex::new(8.0, 12.0),
    ];
    assert_eq!(complex::dotc(4, &x, 1, &y, 1), Complex::new(348.0, -64.0));
    assert_eq!(complex::dotc(4, &x, -1, &y, -1), Complex::new(348.0, -64.0));

    let x = vec![Complex::new(1.0, 5.0)];
    let y = vec![Complex::new(5.0, 9.0)];
    assert_eq!(complex::dotc(0, &x, -1, &y, -1), Complex::new(0.0, 0.0));
}

#[test]
fn dotu() {
    let x = vec![
        Complex::new(1.0, 5.0),
        Complex::new(2.0, 6.0),
        Complex::new(3.0, 7.0),
        Complex::new(4.0, 8.0),
    ];
    let y = vec![
        Complex::new(5.0, 9.0),
        Complex::new(6.0, 10.0),
        Complex::new(7.0, 11.0),
        Complex::new(8.0, 12.0),
    ];
    assert_eq!(complex::dotu(4, &x, 1, &y, 1), Complex::new(-208.0, 284.0));
    assert_eq!(
        complex::dotu(4, &x, -1, &y, -1),
        Complex::new(-208.0, 284.0)
    );

    let x = vec![Complex::new(1.0, 5.0)];
    let y = vec![Complex::new(5.0, 9.0)];
    assert_eq!(complex::dotu(0, &x, -1, &y, -1), Complex::new(0.0, 0.0));
}

#[test]
fn rot() {
    use core::f64::consts::PI;
    let mut x = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)];
    let mut y = vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)];
    complex::rot(
        2,
        &mut x,
        1,
        &mut y,
        1,
        (PI * (1.0 / 6.0)).cos(),
        (PI * (1.0 / 6.0)).sin(),
    );
    assert_eq!(
        x,
        vec![
            Complex::new(0.8660254037844387, 0.0),
            Complex::new(0.49999999999999994, 0.0)
        ]
    );
    assert_eq!(
        y,
        vec![
            Complex::new(-0.49999999999999994, 0.0),
            Complex::new(0.8660254037844387, 0.0)
        ]
    );
    // assert_eq!(x, vec![Complex::new(0.8660254037844387, 0.0), Complex::new(0.500000000000000, 0.0)]);
    // assert_eq!(y, vec![Complex::new(-0.500000000000000, 0.0), Complex::new(0.8660254037844387, 0.0)]);

    let mut x = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)];
    let mut y = vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)];
    complex::rot(
        2,
        &mut x,
        -1,
        &mut y,
        -1,
        (PI * (1.0 / 6.0)).cos(),
        (PI * (1.0 / 6.0)).sin(),
    );
    assert_eq!(
        x,
        vec![
            Complex::new(0.8660254037844387, 0.0),
            Complex::new(0.49999999999999994, 0.0)
        ]
    );
    assert_eq!(
        y,
        vec![
            Complex::new(-0.49999999999999994, 0.0),
            Complex::new(0.8660254037844387, 0.0)
        ]
    );
    // assert_eq!(x, vec![Complex::new(0.8660254037844387, 0.0), Complex::new(0.500000000000000, 0.0)]);
    // assert_eq!(y, vec![Complex::new(-0.500000000000000, 0.0), Complex::new(0.8660254037844387, 0.0)]);

    let mut x = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)];
    let mut y = vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)];
    complex::rot(
        0,
        &mut x,
        -1,
        &mut y,
        -1,
        (PI * (1.0 / 6.0)).cos(),
        (PI * (1.0 / 6.0)).sin(),
    );
    assert_eq!(x, vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
    assert_eq!(y, vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]);
}

#[test]
fn rotg() {
    let mut a = Complex::new(11.0, 19.0);
    let mut b = Complex::new(34.0, 23.0);
    let mut c = 0.0;
    let mut s = Complex::new(0.0, 0.0);
    complex::rotg(&mut a, &mut b, &mut c, &mut s);
    assert_approx_eq_cplx!(a, Complex::new(23.323763103564644, 40.286_499_906_157_11));
    assert_eq!(b, Complex::new(34.000000, 23.0000000));
    assert_approx_eq!(c, 0.471_622_008_470_787_3);
    assert_approx_eq_cplx!(s, Complex::new(0.793_538_275_663_503_1, 0.384_538_276_616_222_8));

    let mut a = Complex::new(0.0, 0.0);
    let mut b = Complex::new(-1.0, -1.0);
    let mut c = 0.0;
    let mut s = Complex::new(0.0, 0.0);
    complex::rotg(&mut a, &mut b, &mut c, &mut s);
    assert_eq!(a, Complex::new(-1.0, -1.0));
    assert_eq!(b, Complex::new(-1.0, -1.0));
    assert_eq!(c, 0.0);
    assert_eq!(s, Complex::new(1.0, 0.0));
}

#[test]
fn sscal() {
    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    complex::sscal(6, 2.0, &mut x, 1);
    assert_eq!(
        x,
        vec![
            Complex::new(2.0, 14.0),
            Complex::new(4.0, 16.0),
            Complex::new(6.0, 18.0),
            Complex::new(8.0, 20.0),
            Complex::new(10.0, 22.0),
            Complex::new(12.0, 24.0)
        ]
    );

    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    complex::sscal(3, 2.0, &mut x, 2);
    assert_eq!(
        x,
        vec![
            Complex::new(2.0, 14.0),
            Complex::new(2.0, 8.0),
            Complex::new(6.0, 18.0),
            Complex::new(4.0, 10.0),
            Complex::new(10.0, 22.0),
            Complex::new(6.0, 12.0)
        ]
    );

    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    let expect = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    complex::sscal(3, 2.0, &mut x, 0);
    assert_eq!(x, expect);

    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    complex::sscal(0, 2.0, &mut x, 1);
    assert_eq!(x, expect);

    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    complex::sscal(6, 0.0, &mut x, 1);
    assert_eq!(
        x,
        vec![
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0)
        ]
    );
}

#[test]
fn scal() {
    let mut x = vec![
        Complex::new(1.262_954_284_880_793_3, -0.928_567_034_713_538_1),
        Complex::new(-0.326_233_360_705_649_4, -0.294_720_446_790_560_2),
        Complex::new(1.329_799_262_922_500_6, -0.005_767_172_747_536_955),
        Complex::new(1.272_429_321_429_404_7, 2.404_653_388_857_951),
        Complex::new(0.414_641_434_456_408_2, 0.763_593_461_140_459_6),
        Complex::new(-1.539_950_041_903_709_5, -0.799_009_248_989_368_2),
    ];
    let a = Complex::new(2.0, 4.0);
    complex::scal(6, a, &mut x, 1);
    assert_eq!(
        x,
        vec![
            Complex::new(6.240_176_708_615_738_5, 3.1946830700960973),
            Complex::new(0.526_415_065_750_942, -1.894_374_336_403_718),
            Complex::new(2.682_667_216_835_149, 5.307_662_706_194_929),
            Complex::new(-7.073_754_912_572_994, 9.899_024_063_433_52),
            Complex::new(-2.225_090_975_649_022, 3.185_752_660_106_552),
            Complex::new(0.11613691215005373, -7.757_818_665_593_574)
        ]
    );

    let mut x = vec![
        Complex::new(1.262_954_284_880_793_3, -0.928_567_034_713_538_1),
        Complex::new(-0.326_233_360_705_649_4, -0.294_720_446_790_560_2),
        Complex::new(1.329_799_262_922_500_6, -0.005_767_172_747_536_955),
        Complex::new(1.272_429_321_429_404_7, 2.404_653_388_857_951),
        Complex::new(0.414_641_434_456_408_2, 0.763_593_461_140_459_6),
        Complex::new(-1.539_950_041_903_709_5, -0.799_009_248_989_368_2),
    ];
    complex::scal(3, a, &mut x, 2);
    assert_eq!(
        x,
        vec![
            Complex::new(6.240_176_708_615_738_5, 3.1946830700960973),
            Complex::new(-0.326_233_360_705_649_4, -0.294_720_446_790_560_2),
            Complex::new(2.682_667_216_835_149, 5.307_662_706_194_929),
            Complex::new(1.272_429_321_429_404_7, 2.404_653_388_857_951),
            Complex::new(-2.225_090_975_649_022, 3.185_752_660_106_552),
            Complex::new(-1.539_950_041_903_709_5, -0.799_009_248_989_368_2)
        ]
    );

    let mut x = vec![
        Complex::new(1.262_954_284_880_793_3, -0.928_567_034_713_538_1),
        Complex::new(-0.326_233_360_705_649_4, -0.294_720_446_790_560_2),
        Complex::new(1.329_799_262_922_500_6, -0.005_767_172_747_536_955),
        Complex::new(1.272_429_321_429_404_7, 2.404_653_388_857_951),
        Complex::new(0.414_641_434_456_408_2, 0.763_593_461_140_459_6),
        Complex::new(-1.539_950_041_903_709_5, -0.799_009_248_989_368_2),
    ];
    complex::scal(0, a, &mut x, 0);
    assert_eq!(
        x,
        vec![
            Complex::new(1.262_954_284_880_793_3, -0.928_567_034_713_538_1),
            Complex::new(-0.326_233_360_705_649_4, -0.294_720_446_790_560_2),
            Complex::new(1.329_799_262_922_500_6, -0.005_767_172_747_536_955),
            Complex::new(1.272_429_321_429_404_7, 2.404_653_388_857_951),
            Complex::new(0.414_641_434_456_408_2, 0.763_593_461_140_459_6),
            Complex::new(-1.539_950_041_903_709_5, -0.799_009_248_989_368_2)
        ]
    );

    let mut x = vec![
        Complex::new(1.262_954_284_880_793_3, -0.928_567_034_713_538_1),
        Complex::new(-0.326_233_360_705_649_4, -0.294_720_446_790_560_2),
        Complex::new(1.329_799_262_922_500_6, -0.005_767_172_747_536_955),
        Complex::new(1.272_429_321_429_404_7, 2.404_653_388_857_951),
        Complex::new(0.414_641_434_456_408_2, 0.763_593_461_140_459_6),
        Complex::new(-1.539_950_041_903_709_5, -0.799_009_248_989_368_2),
    ];
    complex::scal(3, a, &mut x, 0);
    assert_eq!(
        x,
        vec![
            Complex::new(1.262_954_284_880_793_3, -0.928_567_034_713_538_1),
            Complex::new(-0.326_233_360_705_649_4, -0.294_720_446_790_560_2),
            Complex::new(1.329_799_262_922_500_6, -0.005_767_172_747_536_955),
            Complex::new(1.272_429_321_429_404_7, 2.404_653_388_857_951),
            Complex::new(0.414_641_434_456_408_2, 0.763_593_461_140_459_6),
            Complex::new(-1.539_950_041_903_709_5, -0.799_009_248_989_368_2)
        ]
    );
}

#[test]
fn swap() {
    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    let mut y = vec![
        Complex::new(13.0, 74.0),
        Complex::new(23.0, 84.0),
        Complex::new(33.0, 94.0),
        Complex::new(43.0, 104.0),
        Complex::new(53.0, 114.0),
        Complex::new(63.0, 124.0),
    ];

    complex::swap(6, &mut x, 1, &mut y, 1);
    assert_eq!(
        x,
        vec![
            Complex::new(13.0, 74.0),
            Complex::new(23.0, 84.0),
            Complex::new(33.0, 94.0),
            Complex::new(43.0, 104.0),
            Complex::new(53.0, 114.0),
            Complex::new(63.0, 124.0)
        ]
    );
    assert_eq!(
        y,
        vec![
            Complex::new(1.0, 7.0),
            Complex::new(2.0, 8.0),
            Complex::new(3.0, 9.0),
            Complex::new(4.0, 10.0),
            Complex::new(5.0, 11.0),
            Complex::new(6.0, 12.0)
        ]
    );

    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    let mut y = vec![
        Complex::new(13.0, 74.0),
        Complex::new(23.0, 84.0),
        Complex::new(33.0, 94.0),
        Complex::new(43.0, 104.0),
        Complex::new(53.0, 114.0),
        Complex::new(63.0, 124.0),
    ];

    complex::swap(6, &mut x, -1, &mut y, 1);
    assert_eq!(
        x,
        vec![
            Complex::new(63.0, 124.0),
            Complex::new(53.0, 114.0),
            Complex::new(43.0, 104.0),
            Complex::new(33.0, 94.0),
            Complex::new(23.0, 84.0),
            Complex::new(13.0, 74.0)
        ]
    );
    assert_eq!(
        y,
        vec![
            Complex::new(6.0, 12.0),
            Complex::new(5.0, 11.0),
            Complex::new(4.0, 10.0),
            Complex::new(3.0, 9.0),
            Complex::new(2.0, 8.0),
            Complex::new(1.0, 7.0)
        ]
    );

    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    let mut y = vec![
        Complex::new(13.0, 74.0),
        Complex::new(23.0, 84.0),
        Complex::new(33.0, 94.0),
        Complex::new(43.0, 104.0),
        Complex::new(53.0, 114.0),
        Complex::new(63.0, 124.0),
    ];

    complex::swap(6, &mut x, 1, &mut y, -1);
    assert_eq!(
        x,
        vec![
            Complex::new(63.0, 124.0),
            Complex::new(53.0, 114.0),
            Complex::new(43.0, 104.0),
            Complex::new(33.0, 94.0),
            Complex::new(23.0, 84.0),
            Complex::new(13.0, 74.0)
        ]
    );
    assert_eq!(
        y,
        vec![
            Complex::new(6.0, 12.0),
            Complex::new(5.0, 11.0),
            Complex::new(4.0, 10.0),
            Complex::new(3.0, 9.0),
            Complex::new(2.0, 8.0),
            Complex::new(1.0, 7.0)
        ]
    );

    let mut x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    let mut y = vec![
        Complex::new(13.0, 74.0),
        Complex::new(23.0, 84.0),
        Complex::new(33.0, 94.0),
        Complex::new(43.0, 104.0),
        Complex::new(53.0, 114.0),
        Complex::new(63.0, 124.0),
    ];
    complex::swap(0, &mut x, 1, &mut y, -1);
    assert_eq!(
        x,
        vec![
            Complex::new(1.0, 7.0),
            Complex::new(2.0, 8.0),
            Complex::new(3.0, 9.0),
            Complex::new(4.0, 10.0),
            Complex::new(5.0, 11.0),
            Complex::new(6.0, 12.0)
        ]
    );
    assert_eq!(
        y,
        vec![
            Complex::new(13.0, 74.0),
            Complex::new(23.0, 84.0),
            Complex::new(33.0, 94.0),
            Complex::new(43.0, 104.0),
            Complex::new(53.0, 114.0),
            Complex::new(63.0, 124.0)
        ]
    );
}

#[test]
fn iamax() {
    let x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(0.0, 0.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    assert_eq!(complex::iamax(6, &x, 1), 6);

    let x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(2.0, 8.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    assert_eq!(complex::iamax(3, &x, 2), 3);
    assert_eq!(complex::iamax(6, &x, 0), 0);
    assert_eq!(complex::iamax(0, &x, 1), 0);

    let x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(0.0, 1.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    assert_eq!(complex::iamax(1, &x, 1), 1);
}

#[test]
fn asum() {
    let x = vec![
        Complex::new(1.0, 7.0),
        Complex::new(0.0, 0.0),
        Complex::new(3.0, 9.0),
        Complex::new(4.0, 10.0),
        Complex::new(5.0, 11.0),
        Complex::new(6.0, 12.0),
    ];
    assert_eq!(complex::asum(6, &x, 1), 68.0);
    assert_eq!(complex::asum(3, &x, 2), 36.0);
    assert_eq!(complex::asum(0, &x, 2), 0.0);
    assert_eq!(complex::asum(0, &x, 0), 0.0);
}

#[test]
fn nrm2() {
    let x = vec![
        Complex::new(0.0, 0.763_593_461_140_459_6),
        Complex::new(0.0, 0.0),
        Complex::new(1.329_799_262_922_500_6, -1.147_657_009_236_351_4),
        Complex::new(1.272_429_321_429_404_7, -0.28946157368822334),
        Complex::new(0.414_641_434_456_408_2, -0.29921511789731614),
        Complex::new(-1.539_950_041_903_709_5, -0.411_510_832_795_067),
        Complex::new(-0.928_567_034_713_538_1, 0.252_223_448_156_132_3),
        Complex::new(-0.294_720_446_790_560_2, -0.891_921_127_284_568_6),
        Complex::new(-0.005_767_172_747_536_955, 0.43568329935571865),
        Complex::new(2.404_653_388_857_951, -1.237_538_421_929_958),
    ];
    assert_approx!(complex::nrm2(10, &x, 1), 4.181_580_545_299_952);
    assert_eq!(complex::nrm2(0, &x, 1), 0.0);
    assert_eq!(complex::nrm2(10, &x, 0), 0.0);
}
