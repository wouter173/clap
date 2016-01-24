#[cfg(feature = "yaml")]
use yaml_rust::Yaml;

use App;
use ArgMatches;

/// The abstract representation of a command line subcommand used by the consumer of the library.
///
///
/// This struct is used by the library consumer and describes all the valid options of the
/// subcommand for their program. SubCommands are treated like "sub apps" and contain all the same
/// possibilities (such as their own arguments and subcommands).
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg, SubCommand};
/// # let matches = App::new("myprog")
/// #                    .subcommand(
/// SubCommand::with_name("config")
///                .about("Used for configuration")
///                .arg(Arg::with_name("config_file")
///                           .help("The configuration file to use")
///                           .index(1))
/// # ).get_matches();
#[derive(Debug, Clone)]
pub struct SubCommand<'a> {
    #[doc(hidden)]
    pub name: String,
    #[doc(hidden)]
    pub matches: ArgMatches<'a>,
}

impl<'a> SubCommand<'a> {
    /// Creates a new instance of a subcommand requiring a name. Will be displayed
    /// to the user when they print version or help and usage information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let prog = App::new("myprog").subcommand(
    /// SubCommand::with_name("config")
    /// # ).get_matches();
    /// ```
    pub fn with_name<'b>(name: &str) -> App<'a, 'b> {
        App::new(name)
    }

    /// Creates a new instance of a subcommand from a YAML (.yml) document
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg, SubCommand};
    /// let sc_yaml = load_yaml!("test_subcommand.yml");
    /// let sc = SubCommand::from_yaml(sc_yaml);
    /// ```
    #[cfg(feature = "yaml")]
    pub fn from_yaml<'y>(yaml: &'y Yaml) -> App<'y, 'y> {
        App::from_yaml(yaml)
    }
}
