use crate::Options;
use clap::error::ErrorKind;
use clap::Parser;

#[test]
fn dash_dash_help() {
    match get_parse_error_kind(["FAKE_EXECUTABLE", "--help"]) {
        ErrorKind::DisplayHelp => {}
        other => panic!("Unexpected error kind: {:#?}", other),
    }
}

#[test]
fn no_args_help() {
    match get_parse_error_kind(["FAKE_EXECUTABLE"]) {
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {}
        other => panic!("Unexpected error kind: {:#?}", other),
    }
}

fn get_parse_error_kind<const K: usize>(args: [&str; K]) -> ErrorKind {
    Options::try_parse_from(args).err().unwrap().kind()
}
