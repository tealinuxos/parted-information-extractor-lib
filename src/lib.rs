use serde_json::Value;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn get_partition_information() -> Vec<Value> {
    let mut parted = Command::new("sudo")
        .arg("parted")
        .arg("-lj")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let parted_output = parted.stdout.take().expect("Gagal mengambil informasi");
    let mut buffer_read = BufReader::new(parted_output);

    let mut isi_disk: Vec<String> = vec![];

    'mainloop: loop {
        let mut _count_cetakan: i32 = 0;

        let mut isi_json = String::new();
        let mut isi_buffer = String::new();

        loop {
            isi_buffer.clear();
            _count_cetakan += 1;
            let ukuran_buffer = buffer_read.read_line(&mut isi_buffer).unwrap();

            if ukuran_buffer == 0 {
                break 'mainloop;
            }

            if (ukuran_buffer == 2) && (isi_buffer == "}\n") {
                isi_json.push_str(&isi_buffer);
                break;
            }

            isi_json.push_str(&isi_buffer);

        }

        isi_disk.push(isi_json);
    }

    let mut all_json_information: Vec<Value> = vec![];

    for i in isi_disk.iter() {
        let the_json: Value = serde_json::from_str(i).unwrap();
        all_json_information.push(the_json);
    }

    all_json_information
}
