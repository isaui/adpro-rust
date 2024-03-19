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
![Coomit 2 screen capture](/assets/images/run-rush-hello.png)
![Coomit 2 screen capture](/assets/images/get-rush-hello.png)