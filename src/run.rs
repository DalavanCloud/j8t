/*
 * Copyright 2018 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std;
use std::io::Write;

use bind;
use deblock;
use eval;
use gen;
use parse;
use rename;

pub struct Trace {
    log: bool,
    points: Vec<(usize, String)>,
}

impl Trace {
    pub fn new(log: bool) -> Trace {
        Trace {
            log: log,
            points: Vec::new(),
        }
    }

    pub fn measure<R, F: FnMut() -> R>(&mut self, msg: &str, mut f: F) -> R {
        let start = std::time::Instant::now();
        let r = f();
        let dur = std::time::Instant::now().duration_since(start);
        let time = (dur.as_secs() * 1000 + (dur.subsec_nanos() as u64 / 1_000_000)) as usize;
        if self.log {
            eprintln!("{} {}ms", msg, time);
        }
        self.points.push((time, msg.into()));
        r
    }
}

#[derive(PartialEq)]
pub enum Rename {
    Off,
    On,
    Debug,
}

pub struct Invocation {
    pub filename: String,
    pub input: Vec<u8>,
    pub fmt: bool,
    pub rename: Rename,
}

pub fn run(
    trace: &mut Trace,
    inv: Invocation,
    write: &mut Write,
) -> std::result::Result<(), String> {
    let mut p = parse::Parser::new(&inv.input);
    let mut module = match trace.measure("parse", || p.module()) {
        Ok(stmts) => stmts,
        Err(err) => {
            return Err(format!("{}", err.pretty(&p.lexer)));
        }
    };

    let warnings = trace.measure("bind", || bind::bind(&mut module));
    for w in warnings {
        println!("warn: {}", w);
    }

    trace.measure("eval", || eval::eval(&mut module));

    if inv.rename != Rename::Off {
        trace.measure("rename", || {
            rename::rename(&mut module, inv.rename == Rename::Debug)
        });
    }

    trace.measure("deblock", || deblock::deblock(&mut module));

    trace.measure("write", || {
        {
            let mut writer = gen::Writer::new(write);
            writer.disable_asi = inv.fmt;
            writer.module(&module).unwrap();
        }
        write.flush().unwrap();
    });
    Ok(())
}
