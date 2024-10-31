use std::io; // Giriş/Çıkış işlemleri için gerekli kütüphaneyi ekle

// Oyuncu sembolleri
#[derive(PartialEq, Debug)] // Player enum'u, PartialEq ve Debug trait'lerini uygulayarak karşılaştırma ve hata ayıklama için kullanılabilir hale getirir
enum Player {
    X, // X sembolü
    O, // O sembolü
}

// Izgara oluşturma (3x3)
fn create_board() -> Vec<Vec<char>> {
    // 3 satır ve 3 sütundan oluşan boş bir izgarayı temsil eden vektörü oluştur
    vec![vec![' '; 3], vec![' '; 3], vec![' '; 3]]
}

// Izgarayı gösterme
fn print_board(board: &Vec<Vec<char>>) {
    // İzgaradaki her satırı yazdır
    for row in board {
        println!(" {} | {} | {}", row[0], row[1], row[2]); // Satırdaki sembolleri yazdır
        println!("---+---+---"); // Satırlar arasına ayırıcı ekle
    }
}

// Kazanma durumunu kontrol etme
fn check_winner(board: &Vec<Vec<char>>, player: &Player) -> bool {
    // Aktif oyuncunun sembolünü al
    let symbol = match player {
        Player::X => 'X', // X oyuncusu için sembol
        Player::O => 'O', // O oyuncusu için sembol
    };

    // Yatay ve dikey kontrol
    for i in 0..3 {
        // Yatay kazanç kontrolü
        if (board[i][0] == symbol && board[i][1] == symbol && board[i][2] == symbol)
            || (board[0][i] == symbol && board[1][i] == symbol && board[2][i] == symbol)
        {
            return true; // Kazanma durumu varsa true döndür
        }
    }

    // Çapraz kazanç kontrolü
    if (board[0][0] == symbol && board[1][1] == symbol && board[2][2] == symbol)
        || (board[0][2] == symbol && board[1][1] == symbol && board[2][0] == symbol)
    {
        return true; // Kazanma durumu varsa true döndür
    }

    false // Kazanma durumu yoksa false döndür
}

// Oyuncu hamlesi alma
fn get_move(player: &Player) -> (usize, usize) {
    loop {
        let mut input = String::new(); // Kullanıcı girdisini tutacak değişken
        println!("Player {:?}, enter your move as row and column (e.g., 1 1):", player);
        // Oyuncuya hamlesini girmesi için mesaj yazdır
        io::stdin().read_line(&mut input).expect("Failed to read input"); // Girişi oku

        // Girdiyi boşluklara göre ayırıyoruz
        let inputs: Vec<&str> = input.trim().split_whitespace().collect(); // Girişi boşluklardan ayır ve vektör olarak sakla

        // İki giriş olmalı: satır ve sütun
        if inputs.len() == 2 {
            // Girdileri sayıya dönüştür ve sınırları kontrol et
            if let (Ok(row), Ok(col)) = (inputs[0].parse::<usize>(), inputs[1].parse::<usize>()) {
                if row > 0 && row <= 3 && col > 0 && col <= 3 {
                    return (row - 1, col - 1); // 1 tabanlı girişten 0 tabanlı diziye dönüşüm
                }
            }
        }
        println!("Invalid input. Please enter two numbers between 1 and 3."); // Geçersiz giriş durumunda hata mesajı yazdır
    }
}

// Hamleyi işleme
fn make_move(board: &mut Vec<Vec<char>>, row: usize, col: usize, player: &Player) -> bool {
    // Seçilen hücre boşsa hamle yapılabilir
    if board[row][col] == ' ' {
        let symbol = match player {
            Player::X => 'X', // X sembolü
            Player::O => 'O', // O sembolü
        };
        board[row][col] = symbol; // İzgarada oyuncunun sembolünü yerleştir
        true // Hamle başarılı
    } else {
        println!("That spot is already taken. Try again."); // Eğer hücre doluysa hata mesajı
        false // Hamle başarısız
    }
}

fn main() {
    let mut board = create_board(); // Boş izgarayı oluştur
    let mut current_player = Player::X; // İlk oyuncuyu belirle

    for _ in 0..9 { // En fazla 9 hamle yap
        print_board(&board); // İzgarayı yazdır

        let (row, col) = get_move(&current_player); // Oyuncudan hamlesini al
        if make_move(&mut board, row, col, &current_player) { // Hamleyi yap
            if check_winner(&board, &current_player) { // Kazanan kontrolü
                print_board(&board); // İzgarayı yazdır
                println!("Player {:?} wins!", current_player); // Kazananı ilan et
                return; // Programdan çık
            }
            // Oyuncuları değiştir
            current_player = if current_player == Player::X {
                Player::O // Eğer X ise O'ya geç
            } else {
                Player::X // Eğer O ise X'e geç
            };
        }
    }

    println!("It's a tie!"); // Eğer tüm hamleler yapıldıysa ve kazanan yoksa berabere olduğunu ilan et
}
