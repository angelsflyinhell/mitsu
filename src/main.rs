use std::{thread, time::SystemTime};

fn main() {
    println!("Scanning the web... (this may take weeks to complete, online servers will be printed into the console)");

    let pre_calc_time = SystemTime::now();
    scan_ips();
    println!(
        "Scanned the internet in {:?} seconds",
        SystemTime::now().duration_since(pre_calc_time).unwrap()
    );
}

fn scan_ips() {
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];

    for mon_one in (1..223).rev() {
        threads.push(thread::spawn(move || {
            for mon_two in (0..255).rev() {
                for mon_three in (0..255).rev() {
                    for mon_four in (0..255).rev() {
                        // println!(
                        //     "Requesting on: {}.{}.{}.{}:25565\r",
                        //     mon_one, mon_two, mon_three, mon_four
                        // );
                        request_ip(&format!("{}.{}.{}.{}:25565", mon_one, mon_two, mon_three, mon_four));
                    }
                }
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

fn request_ip(ip: &str) {
    let resp = reqwest::blocking::get(&format!("https://api.mcsrvstat.us/2/{}", ip));

    match resp {
        Ok(resp) => {
            let response = resp.text();
            match response {
                Ok(text) => {
                    if text.contains("online\":true") {
                        println!("{}", ip);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }


}
