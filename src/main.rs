use std::io;

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\n===== TODO APP =====");
        println!("1. Lihat Todo");
        println!("2. Tambah Todo");
        println!("3. Hapus Todo");
        println!("4. Keluar");
        println!("Pilih menu (1-4): ");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca input");
        let pilihan = pilihan.trim();

        match pilihan {
            "1" => {
                println!("\n=== Daftar Todo ===");
                if todos.is_empty() {
                    println!("Belum ada todo.");
                } else {
                    for (i, todo) in todos.iter().enumerate() {
                        println!("{}. {}", i + 1, todo);
                    }
                }
            }
            "2" => {
                println!("Masukkan todo baru:");
                let mut new_todo = String::new();
                io::stdin().read_line(&mut new_todo).expect("Gagal membaca input");
                todos.push(new_todo.trim().to_string());
                println!("Todo berhasil ditambahkan!");
            }
            "3" => {
                println!("Masukkan nomor todo yang mau dihapus:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Gagal membaca input");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input tidak valid!");
                        continue;
                    }
                };
                if index == 0 || index > todos.len() {
                    println!("Nomor tidak valid!");
                } else {
                    todos.remove(index - 1);
                    println!("Todo berhasil dihapus!");
                }
            }
            "4" => {
                println!("Keluar dari aplikasi...");
                break;
            }
            _ => println!("Pilihan tidak valid!"),
        }
    }
}
