use super::{NoCacheStrategy, PackageManager, PmMode, PromptStrategy, Strategies};
use crate::dispatch::config::Config;
use crate::error::Error;
use crate::exec::{self, Cmd};
use crate::print::{self, PROMPT_RUN};

pub struct Apk {
    pub cfg: Config,
}

lazy_static! {
    static ref PROMPT_STRAT: Strategies = Strategies {
        prompt: PromptStrategy::CustomPrompt,
        ..Default::default()
    };
    static ref INSTALL_STRAT: Strategies = Strategies {
        prompt: PromptStrategy::CustomPrompt,
        no_cache: NoCacheStrategy::with_flags(&["--no-cache"]),
        ..Default::default()
    };
}

impl PackageManager for Apk {
    /// Get the name of the package manager.
    fn name(&self) -> String {
        "apk".into()
    }

    fn cfg(&self) -> Config {
        self.cfg.clone()
    }

    /// Q generates a list of installed packages.
    fn q(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        if kws.is_empty() {
            self.just_run_default(Cmd::new(&["apk", "info"]).flags(flags))
        } else {
            self.qs(kws, flags)
        }
    }

    /// Qi displays local package information: name, version, description, etc.
    fn qi(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.si(kws, flags)
    }

    /// Ql displays files provided by local package.
    fn ql(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(Cmd::new(&["apk", "info", "-L"]).kws(kws).flags(flags))
    }

    /// Qo queries the package which provides FILE.
    fn qo(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(
            Cmd::new(&["apk", "info", "--who-owns"])
                .kws(kws)
                .flags(flags),
        )
    }

    /// Qs searches locally installed package for names or descriptions.
    // According to https://www.archlinux.org/pacman/pacman.8.html#_query_options_apply_to_em_q_em_a_id_qo_a,
    // when including multiple search terms, only packages with descriptions matching ALL of those terms are returned.
    fn qs(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        let search = |contents: &str| {
            exec::grep(contents, kws)
                .iter()
                .for_each(|ln| println!("{}", ln))
        };

        let search_output = |cmd| {
            let cmd = Cmd::new(cmd).flags(flags);
            if !self.cfg.dry_run {
                print::print_cmd(&cmd, PROMPT_RUN);
            }
            let out_bytes = self.run(cmd, PmMode::Mute, Default::default())?;
            search(&String::from_utf8(out_bytes)?);
            Ok(())
        };

        search_output(&["apk", "info", "-d"])
    }

    /// Qu lists packages which have an update available.
    //? Is that the right way to input '<'?
    fn qu(&self, _kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(Cmd::new(&["apk", "version", "-l", "<"]).flags(flags))
    }

    /// R removes a single package, leaving all of its dependencies installed.
    fn r(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["apk", "del"]).kws(kws).flags(flags),
            Default::default(),
            PROMPT_STRAT.clone(),
        )
    }

    /// Rn removes a package and skips the generation of configuration backup files.
    fn rn(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["apk", "del", "--purge"]).kws(kws).flags(flags),
            Default::default(),
            PROMPT_STRAT.clone(),
        )
    }

    /// Rns removes a package and its dependencies which are not required by any other installed package, and skips the generation of configuration backup files.
    fn rns(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["apk", "del", "--purge", "-r"])
                .kws(kws)
                .flags(flags),
            Default::default(),
            PROMPT_STRAT.clone(),
        )
    }

    /// Rs removes a package and its dependencies which are not required by any other installed package,
    /// and not explicitly installed by the user.
    fn rs(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.r(kws, flags)
    }

    /// S installs one or more packages by name.
    fn s(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["apk", "add"]).kws(kws).flags(flags),
            Default::default(),
            INSTALL_STRAT.clone(),
        )
    }

    /// Sc removes all the cached packages that are not currently installed, and the unused sync database.
    fn sc(&self, _kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["apk", "cache", "-v", "clean"]).flags(flags),
            Default::default(),
            PROMPT_STRAT.clone(),
        )
    }

    /// Scc removes all files from the cache.
    fn scc(&self, _kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["rm", "-vrf", "/var/cache/apk/*"]).flags(flags),
            Default::default(),
            PROMPT_STRAT.clone(),
        )
    }

    /// Si displays remote package information: name, version, description, etc.
    fn si(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(Cmd::new(&["apk", "info", "-a"]).kws(kws).flags(flags))
    }

    /// Sii displays packages which require X to be installed, aka reverse dependencies.
    fn sii(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(Cmd::new(&["apk", "info", "-r"]).kws(kws).flags(flags))
    }

    /// Sl displays a list of all packages in all installation sources that are handled by the packages management.
    fn sl(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(Cmd::new(&["apk", "search"]).kws(kws).flags(flags))
    }

    /// Ss searches for package(s) by searching the expression in name, description, short description.
    fn ss(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(Cmd::new(&["apk", "search", "-v"]).kws(kws).flags(flags))
    }

    /// Su updates outdated packages.
    fn su(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        let cmd = if kws.is_empty() {
            Cmd::new(&["apk", "upgrade"]).kws(kws).flags(flags)
        } else {
            Cmd::new(&["apk", "add", "-u"]).kws(kws).flags(flags)
        };
        self.just_run(cmd, Default::default(), INSTALL_STRAT.clone())
    }

    /// Suy refreshes the local package database, then updates outdated packages.
    fn suy(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        let cmd = if kws.is_empty() {
            Cmd::new(&["apk", "upgrade", "-U", "-a"])
                .kws(kws)
                .flags(flags)
        } else {
            Cmd::new(&["apk", "add", "-U", "-u"]).kws(kws).flags(flags)
        };
        self.just_run(cmd, Default::default(), INSTALL_STRAT.clone())
    }

    /// Sw retrieves all packages from the server, but does not install/upgrade anything.
    fn sw(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["apk", "fetch"]).kws(kws).flags(flags),
            Default::default(),
            PROMPT_STRAT.clone(),
        )
    }

    /// Sy refreshes the local package database.
    fn sy(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run_default(Cmd::new(&["apk", "update"]).kws(kws).flags(flags))?;
        if !kws.is_empty() {
            self.s(kws, flags)?;
        }
        Ok(())
    }

    /// U upgrades or adds package(s) to the system and installs the required dependencies from sync repositories.
    fn u(&self, kws: &[&str], flags: &[&str]) -> Result<(), Error> {
        self.just_run(
            Cmd::new(&["apk", "add", "--allow-untrusted"])
                .kws(kws)
                .flags(flags),
            Default::default(),
            INSTALL_STRAT.clone(),
        )
    }
}
