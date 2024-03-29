# adpro-rust

1.  ## Commit 1 reflection notes
```fn handle_connection(mut stream: TcpStream) { let buf_reader = BufReader::new(&mut stream) ....```
Di dalam fungsi handle_connection, kita membuat BufReader baru yang membungkus TcpStream. BufReader digunakan untuk membaca dari TcpStream dengan lebih efisien karena melakukan buffering.

 ```let http_request: Vec<_> = buf_reader.lines() .map(|result| result.unwrap()) .take_while(|line| !line.is_empty()) .collect(); ```
Di sini, kita membaca request HTTP dari TcpStream. Kita membaca setiap baris dari TcpStream menggunakan buf_reader.lines(), kemudian kita unwrap setiap hasilnya untuk mendapatkan String. Kita menggunakan .take_while() untuk mengambil baris-baris sampai bertemu dengan baris kosong (dalam HTTP, baris kosong menandakan akhir dari header request). Akhirnya, kita mengumpulkan baris-baris tersebut ke dalam sebuah vektor menggunakan .collect().

``` println!("Request: {:#?}", http_request); ```
Akhirnya, kita mencetak request HTTP yang telah kita baca.

Jadi, program ini membuat server sederhana yang dapat menerima koneksi TCP, membaca request HTTP, dan mencetaknya, tetapi belum mengembalikan response kepada client.

2. ## Commit 2 Reflection notes
![Commit 2 screen capture](/assets/images/run-rush-hello.png)
![Commit 2 screen capture](/assets/images/get-rush-hello.png)
Setiap http request akan direturn jawaban dari server berupa html pada "hello.html"

3. ## Commit 3 Reflection notes
![Commit 3 Screen capture](/assets/images/rush-404.png)
Sebelum mencoba mengakses indeks dari ```http_request```, kita memeriksa apakah itu tidak kosong. Jika tidak kosong, kita melanjutkan untuk memeriksa apakah permintaan dimulai dengan GET / . Jika benar, kita mengirimkan konten "hello.html" dengan status 200 OK, jika tidak, kita mengirimkan konten "404.html" dengan status 404 NOT FOUND. Jika ```http_request``` kosong, kita juga mengirimkan konten "404 Not Found".

4. ## Commit 4 Reflection notes
![Commit 4 Screen capture](/assets/images/simulate-rust-slow.png)
Disini ketika ada GET request ke endpoint ```/sleep```, kita melakukan thread sleep selama 10s dulu baru mengirimkan response html "hello.html". Ini mensimulasikan web server single trhreaded yang lambat. 

5. ### Commit 5 Reflection notes
![Commit 5 Screen capture](/assets/images/rust-threadpool.png)
Penjelasan untuk ```src/models/threadpool.rs```:
a. **Import modul yang diperlukan:**
   ```rust
   use std::{sync::{mpsc, Arc, Mutex}, thread};
   ```
Modul `sync` digunakan untuk *synchronization primitives*, termasuk `mpsc` (multi-producer, single-consumer) channel, `Arc` (atomic reference counting), dan `Mutex` (mutual exclusion).

b. **Struktur `ThreadPool`:**
   ```rust
   pub struct ThreadPool {
       workers: Vec<Worker>,
       sender: mpsc::Sender<Job>,
   }
   ```
   Struktur `ThreadPool` yang memiliki dua bidang: `workers`, sebuah vektor yang menyimpan `Worker`, dan `sender`, yang digunakan untuk mengirim pekerjaan ke dalam *thread pool* melalui saluran `mpsc`.

c. **`Job`:**
   ```rust
   type Job = Box<dyn FnOnce() + Send + 'static>;
   ```
   Buat tipe `Job` sebagai sebuah *boxed trait object* yang menunjukkan fungsi yang dapat dieksekusi secara sekali (`FnOnce`), dapat dikirim ke *thread* lain (`Send`), dan memiliki masa pakai `'static`.

d. **Implementasi `ThreadPool`:**
   - `execute`: Method ini digunakan untuk mengeksekusi pekerjaan. Menerima fungsi `F`, membungkusnya dalam `Job`, dan mengirimnya melalui saluran `mpsc`.
   - `new`: Method ini digunakan untuk membuat instance baru dari `ThreadPool`. Ini menginisialisasi saluran `mpsc` dan `Arc<Mutex<mpsc::Receiver<Job>>>`, lalu membuat sejumlah `Worker` baru dan mengumpulkannya bersama dengan *sender* dalam `ThreadPool`.

e. **Implementasi `Worker`:**
   `Worker` adalah *thread* yang sebenarnya mengeksekusi pekerjaan yang diterima oleh `ThreadPool`.
   - `new`: Metode ini menerima `id` dan `receiver`, yang merupakan `Arc<Mutex<mpsc::Receiver<Job>>>`, dan kemudian membuat *thread* baru menggunakan `thread::spawn()`. Dalam loop, *thread* ini mengunci `receiver` menggunakan `Mutex`, menerima pekerjaan dari saluran `mpsc`, dan mengeksekusinya.