use grow_r503_wrapper::{ColorIndex, ControlCode, GrowR503Wrapped};
use serialport::{available_ports, SerialPortType::UsbPort};
use std::{env, time::Duration, u16};

const BAUD_RATE: u32 = 9600 * 6;
const VID: u16 = 0x10C4;
const PID: u16 = 0xEA60;

const SEC: Duration = Duration::from_secs(1);

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let save_at = args[1].parse().expect("Failed to parse saving page");
    assert!(save_at < 200);
    let mut s_port = None;
    for port in available_ports().unwrap() {
        if let UsbPort(p_info) = port.port_type {
            if p_info.vid == VID && p_info.pid == PID {
                s_port = Some(port.port_name);
            }
        }
    }

    let serial_port = serialport::new(s_port.expect("Sensor Not Found"), BAUD_RATE)
        .timeout(SEC)
        .open()
        .expect("Failed to open serial port");

    if let Ok(sensor) = GrowR503Wrapped::new(serial_port, None, None, None) {
        let r = sensor.raw_sensor().check_health();
        eprintln!("check_health: {r:?}");
        if r.is_err() {
            return;
        }
        let _ = sensor.raw_sensor().aura_led_config(
            ControlCode::LightAlwaysOn,
            1,
            ColorIndex::Purple,
            0,
        );
        pause();
        let r = sensor.guardar_huella(save_at);
        println!("guardar_huella: {r:?}");
        pause();
        let r = sensor.checkear_huella();
        println!("check_health: (pagina, puntaje){r:?}");
    }
}
