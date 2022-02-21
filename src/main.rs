mod lib;
mod reactors;
mod vendors;

use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{
    execute,
    style::Print,
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use reactors::*;
use vendors::*;
use lib::number_string;

fn main() {
    //variables
    let mut energy: u128 = 0;
    let mut money: u128 = 0;
    
    let energy_production = thread::spawn(move || {
        let carbon_reactor = Reactor::new(EnergySource::Carbon);

        loop{
            execute!(
                stdout(),
                MoveTo(0,0),
                Clear(ClearType::CurrentLine),
                Print(format!("Energy: {}",number_string(energy))),
            ).unwrap();

            thread::sleep(time::Duration::from_secs(1));
            energy += carbon_reactor.production as u128;
        }
    });

    let money_conversion = thread::spawn(move || {
        let mut vendors: Vec<Vendor> = Vec::new();

        vendors.push(Vendor::new(VendorLevel::Shop));
        loop{
            execute!(
                stdout(),
                MoveTo(0,1),
                Clear(ClearType::CurrentLine),
                Print(format!("Money: {}",number_string(money))),
            ).unwrap();

            thread::sleep(time::Duration::from_secs(2));
            money += 2;
        }
    });

    let flush_thread = thread::spawn(|| {
        stdout().flush().unwrap();
        //thread::sleep(time::Duration::from_secs(1));
    });
    
    energy_production.join().unwrap();
    money_conversion.join().unwrap();
    flush_thread.join().unwrap();
}
