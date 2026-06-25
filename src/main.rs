mod scan;
mod ports;
use std::{env};
use rayon::prelude::*;
use ports::TOP_PORTS;
use crate::scan::scan;

fn main(){
    rayon::ThreadPoolBuilder::new()
    .num_threads(50)
    .build_global().unwrap();

    const HELP: &str = "🦀️🎯️🌐️🔥️ The Simple Port Scanner With Rust - rscan 🔥️🌐️🎯️🦀️\n\n\
                        ❗️ Usage:\n\trscan <IP> [OPTIONS]\n\n\
                        ⚠️  Options:\n\
                          \t-p\t--port\t\tScan Specific Ports <80,443,8080,9001>\n\
                          \t-a\t--all\t\t1-65535\n\
                        \n🌐️  By default, scans the top 1000 common ports";
    
    let args : Vec<String> = env::args().collect();

    let help = String::from("--help");

    if args.contains(&help){
        println!("{HELP}");
        return;
    }

    if args.len() < 2 {
        println!("{HELP}");
        return;
    }
    let ip = args[1].trim().to_string();

    
    TOP_PORTS.par_iter().for_each(|port| {
            scan( &ip, *port);    

    });

}

/* 🌐️🔥️✨️♥️⛔️🚫️🦀️⚠️📛️✅️⁉️❌️❗️🎯️ */