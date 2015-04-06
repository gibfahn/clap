use std::collections::HashSet;
use std::collections::BTreeSet;

/// `PosArg` represents a positional argument, i.e. one that isn't preceded 
/// by a `-` or `--`. `PosArg` isn't directly used by the end application
/// writer, only internally to the `clap` library.
///
/// # Example 
///
/// ```sh
/// $ myprog some_file
/// ```
///
/// where `some_file` is the first positional argument to `myprog`
///
/// **NOTE:** The index starts at `1` **NOT** `0`
pub struct PosArg {
    /// The unique name of the argument, required
    pub name: String,
    /// How many occurences of this option have been found when parsing
    pub occurrences: u8,
    /// The value provided to the argument by the user
    pub values: Vec<String>
}

pub struct PosBuilder<'n> {
    pub name: &'n str,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'n str>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// Allow multiple occurrences of an option argument such as "-c some -c other"
    pub multiple: bool,
    /// A list of names of other arguments that are *required* to be used when 
    /// this flag is used
    pub requires: Option<HashSet<&'n str>>,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<HashSet<&'n str>>,
    /// A list of possible values for this argument
    pub possible_vals: Option<BTreeSet<&'n str>>,
    /// The index of the argument
    pub index: u8 
}