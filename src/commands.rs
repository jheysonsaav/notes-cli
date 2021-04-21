mod new;

use clap::{App, ArgMatches};
pub use new::NewCommand;

pub trait Command {
    fn command() -> App<'static, 'static>;
    fn setup(args: &ArgMatches);
}
