use rdev::{listen, Key};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn delete_last_char() -> Result<(), String> {
    for _ in 0..2 {
        let status = Command::new("xdotool")
            .arg("key")
            .arg("BackSpace")
            .status()
            .expect("failed to execute xdotool");

        if !status.success() {
            return Err("Error al ejecutar BackSpace".to_string());
        }

        // Introduce un pequeño retardo entre pulsaciones
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

fn write_accented_o() -> Result<(), String> {
    let status = Command::new("xdotool")
        .arg("type")
        .arg("ó")
        .status()
        .expect("failed to execute xdotool");

    if !status.success() {
        return Err("Error al escribir ó".to_string());
    }

    Ok(())
}

fn main() {
    println!("Programa iniciado. Escribe 'o' y presiona espacio para escribir 'ó'. Presiona 'Esc' para salir.");

    let (tx, rx) = channel();

    // Listener para capturar las teclas
    thread::spawn(move || {
        if let Err(e) = listen(move |event| {
            if let rdev::EventType::KeyPress(key) = event.event_type {
                tx.send(key).unwrap();
            }
        }) {
            println!("Error al escuchar eventos: {:?}", e);
        }
    });

    let mut last_key = None;

    loop {
        let key = rx.recv().unwrap();

        match key {
            Key::Space => {
                if let Some(k) = last_key {
                    if k == Key::KeyO {
                        // Borrar la 'o' escrita anteriormente con múltiples pulsaciones
                        delete_last_char().unwrap();

                        // Escribir "ó" usando xdotool
                        write_accented_o().unwrap();

                        // Salir del programa después de escribir "ó"
                        break;
                    }
                }
            }
            Key::Escape => {
                println!("Saliendo del programa.");
                break;
            }
            _ => {
                last_key = Some(key);
            }
        }
    }
}
