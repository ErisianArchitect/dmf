use dmf::string::parsing::{self, *};

pub fn state_machine() -> String {
    struct Foo {
        index: u32,
        count: u32,
    }
    
    struct PrintLoop {
        index: u32,
        count: u32,
        print: &'static str,
    }
    enum State {
        AddUntil(Foo, Box<dyn FnOnce(Foo) -> State + 'static>),
        Return(String),
        // RevertToZero(Foo, Box<dyn FnOnce(Foo) -> State + 'static>),
        PrintLoop(PrintLoop, Box<dyn FnOnce() -> State + 'static>),
    }
    impl State {
        fn add_until<F: FnOnce(Foo) -> State + 'static>(foo: Foo, transition: F) -> Self {
            Self::AddUntil(foo, Box::new(transition))
        }
        
        // fn revert_to_zero<F: FnOnce(Foo) -> State + 'static>(foo: Foo, transition: F) -> Self {
        //     Self::RevertToZero(foo, Box::new(transition))
        // }
        
        fn print_loop<F: FnOnce() -> State + 'static>(count: u32, print: &'static str, transition: F) -> Self {
            Self::PrintLoop(
                PrintLoop {
                    index: 0,
                    count,
                    print,
                },
                Box::new(transition)
            )
        }
    }
    enum Transition {
        State(State),
        Return(String),
    }
    fn run_machine(state: State) -> Transition {
        Transition::State(match state {
            State::AddUntil(mut foo, fn_once) => {
                if foo.index == foo.count {
                    fn_once(foo)
                } else {
                    println!("Adding: {} + 1", foo.index);
                    foo.index += 1;
                    State::AddUntil(foo, fn_once)
                }
            },
            State::Return(foo) => return Transition::Return(foo),
            // State::RevertToZero(mut foo, fn_once) => {
            //     if foo.index == 0 {
            //         fn_once(foo)
            //     } else {
            //         foo.index -= 1;
            //         State::RevertToZero(foo, fn_once)
            //     }
            // },
            State::PrintLoop(mut print_loop, fn_once) => {
                if print_loop.index == print_loop.count {
                    fn_once()
                } else {
                    print_loop.index += 1;
                    println!("{}", print_loop.print);
                    State::PrintLoop(print_loop, fn_once)
                }
            },
        })
    }
    let mut state = State::add_until(Foo { index: 0, count: 10 }, |_| State::print_loop(5, "hello, world!", || State::print_loop(3, "boyi bolomi!", || State::Return(String::from("This is a test.")))));
    loop {
        state = match run_machine(state) {
            Transition::State(state) => state,
            Transition::Return(ret) => return ret,
        }
    }
}

pub fn main() {
    state_machine();
    return;
    println!("Started.");
    let mut parser = Parser::new(include_str!("parse_test_code.oct"));
    assert!(parser.match_exact("use"));
    parser.eat_whitespace().expect("Failed to eat whitespace.");
    let (ident, _) = parser.match_str_fn(ascii_ident()).expect("Failed to read ident.");
    assert_eq!(ident, "std");
    assert!(parser.match_exact("::"));
    let (ident, _) = parser.match_str_fn(ascii_ident()).expect("Failed to read ident.");
    assert_eq!(ident, "collections");
    assert!(parser.match_exact("::"));
    let (ident, _) = parser.match_str_fn(ascii_ident()).expect("Failed to read ident.");
    assert_eq!(ident, "HashMap");
    assert!(parser.match_exact(";"));
    parser.eat_whitespace().expect("Failed to eat whitespace.");
    assert!(parser.match_exact("fn"));
    parser.eat_whitespace().expect("No whitespace.");
    assert!(parser.match_exact("main"));
    assert!(parser.match_exact("()"));
    parser.eat_whitespace().expect("Failed to eat whitespace.");
    assert!(parser.match_exact_char('{'));
    parser.eat_whitespace().expect("Failed to eat whitespace.");
    assert!(parser.match_exact_char('}'));
    assert!(parser.at_end());
    println!("Ended");
    
}