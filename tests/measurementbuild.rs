pub mod utils;

struct BuildArgs<'a> {
    api_major: &'a str,
    api_minor: &'a str,
    build_id: &'a str,
    policy: &'a str,
    nonce: &'a str,
    tik: &'a str,

    launch_digest: Option<&'a str>,

    outfile: Option<&'a str>,
}

fn run_build(args: &BuildArgs) -> String {
    let tik = utils::cargo_root_path(&args.tik);

    let mut sevctl_args = vec![
        "measurement",
        "build",
        "--api-major",
        args.api_major,
        "--api-minor",
        args.api_minor,
        "--build-id",
        args.build_id,
        "--policy",
        args.policy,
        "--nonce",
        args.nonce,
        "--tik",
        &tik,
    ];

    if let Some(ld) = args.launch_digest {
        sevctl_args.push("--launch-digest");
        sevctl_args.push(ld);
    }
    if let Some(val) = &args.outfile {
        sevctl_args.push("--outfile");
        sevctl_args.push(val);
    }

    utils::run_sevctl(&sevctl_args[..]).trim().to_string()
}

fn test_build(expected: &str, args: BuildArgs) {
    let mut output = run_build(&args);
    if let Some(val) = args.outfile {
        output = base64::encode(std::fs::read(val).unwrap());
    }
    assert_eq!(expected, output);
}

#[test]
fn measurement_build() {
    let stdargs = BuildArgs {
        api_major: "0x01",
        api_minor: "40",
        build_id: "40",
        policy: "0x03",
        nonce: "wxP6tRHCFrFQWxsuqZA8QA==",
        tik: "tests/data/measurement/tik1.bin",
        launch_digest: None,
        outfile: None,
    };

    // Test manually passed in --launch-digest
    let args1 = BuildArgs {
        launch_digest: Some("xkvRAfyaSizgonxAjZIAkR8TmolUabBKQKb6KJCDDSM="),
        ..stdargs
    };
    let expected = "lswbxWxI9gckya16JQvdFtpYmNO4b+3inAPpqsgoBI4=";
    test_build(expected, args1);

    // Same as args1 test, but with --outfile.
    let file = tempfile::NamedTempFile::new().unwrap();
    let args_outfile = BuildArgs {
        launch_digest: Some("xkvRAfyaSizgonxAjZIAkR8TmolUabBKQKb6KJCDDSM="),
        outfile: Some(file.path().to_str().unwrap()),
        ..stdargs
    };
    test_build(expected, args_outfile);
}