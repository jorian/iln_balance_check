#![feature(slice_partition_dedup)]

use csv;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::io;
use komodo_rpc_client::arguments::address::Address;
use komodo_rpc_client::{Client, Chain, KomodoRpcApi};
use std::collections::HashMap;

fn main() {
    let mut address_vec = read_addresses_from_file(Path::new("file.csv")).unwrap();
    let mut addresslist = komodo_rpc_client::arguments::AddressList::new();
    // for each address, get the balance and write it into a new csv file.
    address_vec.sort();

    let (dedup, duplicates) = address_vec.partition_dedup_by(|a, b| {
        a == b
    });

    dbg!(duplicates);

    let client = Client::new_assetchain_client(&Chain::Custom(String::from("ILN")))
        .expect("an ILN client was expected");

    let mut hashm = HashMap::new();

    dedup.iter().for_each(|address| {
        let address_list = komodo_rpc_client::arguments::AddressList::from(address);
        let bal = client.get_address_balance(&address_list);

        match bal {
        Ok(balance) => {
            hashm.insert(address.clone(), (balance.balance as f64) / 100_000_000.0);
        }, //println!("{:#?}", balance),
            Err(e) => println!("{:#?}:{:?}", e.to_string(), address_list)
        }
    });
    let mut writer = csv::Writer::from_path(format!("./addresses_with_balance.csv"))
        .expect("Problem while creating CSV writer");

    hashm.iter().for_each(|(key, value)| {
        writer.serialize((key, value)).expect("Error during serialization")
    });
}

fn read_addresses_from_file(addresses: &Path) -> io::Result<Vec<String>>  {
    let file = File::open(addresses)?;
    let reader = BufReader::new(file);

    let mut vec = vec![];

    reader
        .lines()
        .for_each(|line| {
            let str_add = line.unwrap();
            match Address::from(&str_add) {
                Ok(address) => vec.push(address.to_string()),
                Err(err) => println!("error parsing address {}: {}", &str_add, err.to_string())
            }
        });

    Ok(vec)
}