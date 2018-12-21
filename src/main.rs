// ランダムムーブ
extern crate rand;
use rand::Rng;

use std::str;

fn main() {
    // utf-8 にするコマンド。
    // chcp 65001

    // 日本語のUnicode一覧
    // http://orange-factory.com/sample/utf8/code3.html
    // http://orange-factory.com/sample/utf8/code3/e4.html#CJKUnifiedIdeographs
    // http://charset.7jp.net/unicode2.html

    // https://qiita.com/4hiziri/items/dd9800ad7be42c395082

    // let bytes: &[u8] = "亜".as_bytes();
    // let bytes: &[u8] = "京".as_bytes();
    // let bytes: &[u8] = "佐".as_bytes();
    // let bytes: &[u8] = "🀀".as_bytes();
    // let bytes: &[u8] = "森".as_bytes();

    // 番号が飛んでいて、むずかしい。
    // \x{2E80}-\x{2FDF}
    // 々
    // 〇
    // 〻
    // \x{3400}-\x{4DBF}
    // \x{4E00}-\x{9FFF}
    // \x{F900}-\x{FAFF}
    // \x{20000}-\x{2FFFF}

    // こんな感じで作っていけば良さげ☆
    let bytes: &[u8] = &match rand::thread_rng().gen_range(0, 1+1)
    {
        0 => {
            // E4B880～E4B8B0
            let six_bytes:u64 = rand::thread_rng().gen_range(0xE4B880, 0xE4B8B0+1);
            vec![
                ((six_bytes >> 16) & 0xFF) as u8,
                ((six_bytes >> 8) & 0xFF) as u8,
                (six_bytes & 0xFF) as u8
            ]
        },
        _ => {
            // E4B980～E4B9B0
            let six_bytes:u64 = rand::thread_rng().gen_range(0xE4B980, 0xE4B9B0+1);
            vec![
                ((six_bytes >> 16) & 0xFF) as u8,
                ((six_bytes >> 8) & 0xFF) as u8,
                (six_bytes & 0xFF) as u8
            ]
        },
    };

    /*
    // E4B880～E4B8B0
    let six_bytes:u64 = rand::thread_rng().gen_range(0xE4B880, 0xE4B8B0+1);
    let bytes: &[u8] = &vec![
        ((six_bytes >> 16) & 0xFF) as u8,
        ((six_bytes >> 8) & 0xFF) as u8,
        (six_bytes & 0xFF) as u8
    ];
     */
    /*
    let bytes: &[u8] = &vec![
        0xE4,
        0xB8,
        rand::thread_rng().gen_range(0x80, 0x8F+1)
    ];
     */
    // Unicode から吸い出すより、漢字一覧から取ってきた方がいいのか？

    // 漢字の正規表現
    // https://tama-san.com/kanji-regex/



    // http://charset.7jp.net/unicode2.html
    // https://code.i-harness.com/ja/q/14d834
    // https://lets-emoji.com/emojilist/emojilist-28/
    // https://seiai.ed.jp/sys/text/java/utf8table.html
    
    // let bytes = vec![0x4e9c];
    // let bytes = vec![0x4e, 0x9c];
    // let bytes = vec![0x9c, 0x4e, 0x00, 0x00];
    // let bytes = vec![0x00, 0x06, 0x2F, 0xF0];
    // let bytes = vec![0x4E];
    // N? let bytes = vec![0x00, 0x4E, 0x00, 0x00];
    // N？ let bytes = vec![0x00, 0x00, 0x4E, 0x00];
    // ハート？ let bytes = vec![240, 159, 146, 150];

    let unicode = match str::from_utf8( &bytes ) {
        Ok(t) => {t},
        Err(e) => {panic!("Error {}", e)},
    };
    // let unicode = str::from_utf8( rand::thread_rng().gen_range(0x4E00, 0x62FF0+1) ).unwrap();

    println!("bytes.len(): {0}.", bytes.len());
    println!("{0} vec![0x{1:x}, 0x{2:x}, 0x{3:x}].", unicode, bytes[0], bytes[1], bytes[2]);
    // 亜 vec![0xe4, 0xba, 0x9c].
    // 京 vec![0xe4, 0xba, 0xac].
    // 佐 vec![0xe4, 0xbd, 0x90].

    // println!("Hello, world! {0}", unicode);
    // println!("Hello, world! \u{4e00}");
    // println!("Hello, world! \u{1F498}");
    // println!("Hello, world! \u{4e9c}"); // 亜 (UTF-8)
    // println!("Hello, world! \u{54c0}"); // 哀
}
