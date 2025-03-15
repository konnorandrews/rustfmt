pub fn cli() -> Command {
    subcommand("uninstall")
        .about("Remove a Rust binary")
        .arg(
            Arg::new("spec")
                .value_name("SPEC")
                .num_args(0..)
                .add::<clap_complete::ArgValueCandidates>(clap_complete::ArgValueCandidates::new(
                    || get_installed_crates(),
                )),
        )
        .arg(opt("root", "Directory to uninstall packages from").value_name("DIR"))
        .arg_silent_suggestion()
        .arg_package_spec_simple("Package to uninstall")
        .arg(
            multi_opt("bin", "NAME", "Only uninstall the binary NAME")
                .help_heading(heading::TARGET_SELECTION),
        )
        .after_help(color_print::cstr!(
            "Run `<cyan,bold>cargo help uninstall</>` for more detailed information.\n"
        ))
}
