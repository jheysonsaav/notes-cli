mod new;

use clap::{App, ArgMatches};
pub use new::NewCommand;

pub trait Command<'a, 'b> {
    fn command() -> App<'a, 'b>;
    fn setup(args: &ArgMatches);
}
