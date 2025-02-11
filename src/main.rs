/*

RUST several tests

---  https://www.endbasic.dev/docs.html

bool_var? = TRUE
double_var# = 5.0
integer_var% = 5
string_var$ = "Hello, world!"

DIM b AS BOOLEAN ' b? is set to FALSE.
DIM d AS DOUBLE ' d# is set to 0.0.
DIM i AS INTEGER ' i% is set to 0.
DIM s AS STRING ' s$ is set to "".

' Define a 2-dimensional array (aka matrix) with 5 rows and 3 columns.
DIM arr(5, 3) AS INTEGER

DIM arr(10)
FOR i = LBOUND%(arr) TO UBOUND%(arr)
    PRINT i, arr(i)
NEXT

IF (a AND &b_0001) <> 0 THEN PRINT "a has the right-most bit set"
*/

use std::time::{SystemTime,Instant};
use chrono::{DateTime, Local};
//use std::cell::RefCell;
//use std::rc::Rc;


use endbasic_core::{
    ast::Value,
    exec::StopReason,
    syms::Symbol,
};
//use endbasic_core::exec::Machine;

use futures_lite::future::block_on;
use std::fs::File;
use std::io::Read;

/// Converts the raw bytes of a demo file into the program string to expose.
///
/// The input `bytes` must be valid UTF-8, which we can guarantee because these bytes come from
/// files that we own in the source tree.
///
/// On Windows, the output string has all CRLF pairs converted to LF to ensure that the reported
/// demo file sizes are consistent across platforms.
// fn process_demo(bytes: &[u8]) -> String {
//     let raw_content = String::from_utf8_lossy(bytes).to_string();   //.expect("Malformed demo file");
//     if cfg!(target_os = "windows") {
//         raw_content.replace("\r\n", "\n")
//     } else {
//         raw_content.to_owned()
//     }
// }

fn main() {
    let vf = false;

    let args: Vec<String> = std::env::args().collect();
    let bas_file = if args.len() > 1 {
        &args[1]
    } else {
        "fibo.bas"
    };
    std::env::set_var("RUST_LOG", "ru_test1"); // log ALL
    env_logger::init();

    log::info!("Reading BASIC file: {}", bas_file);
    let mut file = File::open(bas_file).expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read file");
    let mut input = content.as_bytes();
    if vf {
        println!("code:\n{}\n----------------------------",content);
    }

    let tnow = Instant::now();
    //let mut machine = Machine::default();

    let builder = endbasic_std::MachineBuilder::default();  // this allready has a trivial console PRINT
    //builder = builder.with_console( Rc::from(RefCell::from(endbasic_std::console::TrivialConsole::default())) );
    let mut machine = builder.build().expect("failed to create interpreter");

    use endbasic_core::ast::ExprType;
    let var_ref = endbasic_core::ast::VarRef::new("MyName", Some(ExprType::Text));
    let v1 = String::from("Banana");
    let _ = machine.get_mut_symbols().set_var(&var_ref, Value::Text(v1));

    // Execute the input script.  All this script can do is modify the state of the machine itself.
    // In other words: the script can set variables in the machine's environment, but that's it.
    loop {
        match block_on(machine.exec(&mut input)).expect("Execution error") {
            StopReason::Eof => break,
            StopReason::Exited(i) => println!("Script explicitly exited with code {}", i),
            StopReason::Break => (), // Ignore signals.
        }
    }

    let took = tnow.elapsed();

    // Now that our script has run, inspect the variables it set on the machine.
    match machine.get_symbols().get_auto("res1") {
        Some(Symbol::Variable(Value::Integer(i))) => {
            log::info!("VAR res1 is {}", i)
        }
        _ => log::error!("Input did not contain res1 var or is of an invalid type"),
    }

    let now2 = SystemTime::now();
    let dt1: DateTime<Local> = DateTime::from(now2);
    log::warn!("Took: {took:?} now: {dt1}");
    log::debug!("THE END!");
}
