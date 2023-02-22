extern crate menu;

use menu::*;
use pancurses::{endwin, initscr, noecho, Input};
use std::fmt::Write;

const ROOT_MENU: Menu<Output> = Menu {
    label: "root",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: select_foo,
                parameters: &[
                    Parameter::Mandatory {
                        parameter_name: "a",
                        help: Some("This is the help text for 'a'"),
                    },
                    Parameter::Optional {
                        parameter_name: "b",
                        help: None,
                    },
                    Parameter::Named {
                        parameter_name: "verbose",
                        help: None,
                    },
                    Parameter::NamedValue {
                        parameter_name: "level",
                        argument_name: "INT",
                        help: Some("Set the level of the dangle"),
                    },
                ],
            },
            command: "foo",
            help: Some(
                "Makes a foo appear.

This is some extensive help text.

It contains multiple paragraphs and should be preceeded by the parameter list.
",
            ),
        },
        &Item {
            item_type: ItemType::Callback {
                function: select_bar,
                parameters: &[],
            },
            command: "bar",
            help: Some("fandoggles a bar"),
        },
        &Item {
            item_type: ItemType::Menu(&Menu {
                label: "sub",
                items: &[
                    &Item {
                        item_type: ItemType::Callback {
                            function: select_baz,
                            parameters: &[],
                        },
                        command: "baz",
                        help: Some("thingamobob a baz"),
                    },
                    &Item {
                        item_type: ItemType::Callback {
                            function: select_quux,
                            parameters: &[],
                        },
                        command: "quux",
                        help: Some("maximum quux"),
                    },
                ],
                entry: Some(enter_sub),
                exit: Some(exit_sub),
            }),
            command: "sub",
            help: Some("enter sub-menu"),
        },
    ],
    entry: Some(enter_root),
    exit: Some(exit_root),
};

struct Output(pancurses::Window);

impl std::fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.0.printw(s);
        Ok(())
    }
}

fn main() {
    let window = initscr();
    window.scrollok(true);
    noecho();
    let mut buffer = [0u8; 64];
    let mut r = Runner::new(&ROOT_MENU, &mut buffer, Output(window));
    loop {
        match r.context.0.getch() {
            Some(Input::Character('\n')) => {
                r.input_byte(b'\r');
            }
            Some(Input::Character(c)) => {
                let mut buf = [0; 4];
                for b in c.encode_utf8(&mut buf).bytes() {
                    r.input_byte(b);
                }
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                r.context.0.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }
    endwin();
}

fn enter_root(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In enter_root").unwrap();
}

fn exit_root(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In exit_root").unwrap();
}

fn select_foo<'a>(_menu: &Menu<Output>, item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_foo. Args = {:?}", args).unwrap();
    writeln!(
        context,
        "a = {:?}",
        ::menu::argument_finder(item, args, "a")
    )
    .unwrap();
    writeln!(
        context,
        "b = {:?}",
        ::menu::argument_finder(item, args, "b")
    )
    .unwrap();
    writeln!(
        context,
        "verbose = {:?}",
        ::menu::argument_finder(item, args, "verbose")
    )
    .unwrap();
    writeln!(
        context,
        "level = {:?}",
        ::menu::argument_finder(item, args, "level")
    )
    .unwrap();
    writeln!(
        context,
        "no_such_arg = {:?}",
        ::menu::argument_finder(item, args, "no_such_arg")
    )
    .unwrap();
}

fn select_bar<'a>(_menu: &Menu<Output>, _item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_bar. Args = {:?}", args).unwrap();
}

fn enter_sub(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In enter_sub").unwrap();
}

fn exit_sub(_menu: &Menu<Output>, context: &mut Output) {
    writeln!(context, "In exit_sub").unwrap();
}

fn select_baz<'a>(_menu: &Menu<Output>, _item: &Item<Output>, args: &[&str], context: &mut Output) {
    writeln!(context, "In select_baz: Args = {:?}", args).unwrap();
}

fn select_quux<'a>(
    _menu: &Menu<Output>,
    _item: &Item<Output>,
    args: &[&str],
    context: &mut Output,
) {
    writeln!(context, "In select_quux: Args = {:?}", args).unwrap();
}

