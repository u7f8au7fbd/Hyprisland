//マクロを指定
#[macro_export]
macro_rules! cmd {
    (utf8) => {
        use std::process::Command;
        Command::new("cmd")
            .args(["/C", "chcp 65001"])
            .output()
            .expect("UTF-8に設定できませんでした");
    };
    (clear) => {
        use std::process::Command;
        Command::new("cmd")
            .args(["/C", "cls"])
            .output()
            .expect("コンソールをリセットできませんでした");
    };
    (line) => {
        println!("----------------------------------------------------------------")
    };
    (red_line) => {
        println!("----------------------------------------------------------------",)
    };
    (green_line) => {
        println!("----------------------------------------------------------------")
    };
    (red_bg) => {
        let ps_script = r#"
        $console = $host.UI.RawUI
        $console.BackgroundColor = 'DarkRed'
        $colors = $host.PrivateData
        $colors.VerboseBackgroundColor = 'DarkRed'
        $colors.WarningBackgroundColor = 'DarkRed'
        $colors.ErrorBackgroundColor = 'DarkRed'
        Clear-Host
    "#;

        std::process::Command::new("powershell")
            .arg("-Command")
            .arg(ps_script)
            .output()
            .expect("failed to execute powershell command");
    };
    (green_bg) => {
        let ps_script = r#"
        $console = $host.UI.RawUI
        $console.BackgroundColor = 'DarkGreen'
        $colors = $host.PrivateData
        $colors.VerboseBackgroundColor = 'DarkGreen'
        $colors.WarningBackgroundColor = 'DarkGreen'
        $colors.ErrorBackgroundColor = 'DarkGreen'
        Clear-Host
    "#;

        std::process::Command::new("powershell")
            .arg("-Command")
            .arg(ps_script)
            .output()
            .expect("failed to execute powershell command");
    };
}

#[macro_export]
macro_rules! format_path {
    ($path:expr) => {
        $path.replace(
            |c: char| {
                !c.is_ascii_alphanumeric()
                    && c != '\\'
                    && c != '/'
                    && c != '<'
                    && c != '>'
                    && c != ':'
                    && c != '?'
                    && c != '*'
                    && c != '|'
                    && c != '"'
            },
            "",
        )
    };
}

#[macro_export]
macro_rules! ini_dir {
    ($path:expr) => {
        use std::{fs, path};
        let exists = path::Path::new($path).exists();
        if exists {
            fs::remove_dir_all($path).expect("ディレクトリを削除できませんでした");
            fs::create_dir_all($path).expect("ディレクトリを作成できませんでした");
        } else {
            fs::create_dir_all($path).expect("ディレクトリを作成できませんでした");
        }
    };
}

#[macro_export]
macro_rules! time_count {
    ($block:block) => {
        {
            cmd!(utf8);
            // 処理開始前の時刻を記録します。
            let start = std::time::Instant::now();
            // ユーザーが提供したコードブロックを実行
            $block
            // 処理終了後の時刻を記録します。
            let end = std::time::Instant::now();
            // 開始時刻と終了時刻の差分（処理時間）を計算
            let duration = end.duration_since(start);
            // 処理時間を出力
            println!("処理にかかった時間: {:?}", duration.as_secs_f64());
            let elapsed = duration.as_secs() / 60;
            println!("経過時間: {}分", elapsed);
        }
    };
}

#[macro_export]
macro_rules! black {
    ($s:expr) => {
        format!("\x1b[30m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! red {
    ($s:expr) => {
        format!("\x1b[31m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! green {
    ($s:expr) => {
        format!("\x1b[32m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! yellow {
    ($s:expr) => {
        format!("\x1b[33m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! blue {
    ($s:expr) => {
        format!("\x1b[34m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! magenta {
    ($s:expr) => {
        format!("\x1b[35m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! cyan {
    ($s:expr) => {
        format!("\x1b[36m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! white {
    ($s:expr) => {
        format!("\x1b[37m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! black_b {
    ($s:expr) => {
        format!("\x1b[40m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! red_b {
    ($s:expr) => {
        format!("\x1b[41m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! green_b {
    ($s:expr) => {
        format!("\x1b[42m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! yellow_b {
    ($s:expr) => {
        format!("\x1b[43m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! blue_b {
    ($s:expr) => {
        format!("\x1b[44m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! magenta_b {
    ($s:expr) => {
        format!("\x1b[45m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! cyan_b {
    ($s:expr) => {
        format!("\x1b[46m{}\x1b[0m", $s)
    };
}
#[macro_export]
macro_rules! white_b {
    ($s:expr) => {
        format!("\x1b[47m{}\x1b[0m", $s)
    };
}

#[macro_export]
macro_rules! clear {
    ($s:expr) => {
        format!("\x1b[0m", $s)
    };
}

#[macro_export]
macro_rules! mold_html {
    ($input:expr) => {
        {
            const CYAN: &str = "\x1b[36m";
            const RESET: &str = "\x1b[0m";
            const GREEN: &str = "\x1b[32m";
            const BLUE: &str = "\x1b[34m";
            const YELLOW: &str = "\x1b[33m";
            // 効率を上げるため、初期容量を入力の長さに基づいて確保
            let mut result = String::with_capacity($input.len() * 2);
            let mut in_brackets = false; // '<'と'>'の間かどうか
            let mut in_quotes = false; // '"'の中かどうか
            let mut first_word = true; // 最初の単語かどうか

            let iter = $input.chars().peekable(); // peekableにして次の文字を確認できるようにする

            for c in iter {
                match c {
                    '<' => {
                        result.push_str(CYAN); // '<'をシアンで表示
                        result.push(c);
                        in_brackets = true; // ブラケット内に入ったフラグを立てる
                        first_word = true; // 最初の単語のフラグをリセット
                    }
                    '>' => {
                        if in_brackets {
                            result.push_str(RESET); // ブラケット内の文字の色をリセット
                        }
                        result.push_str(CYAN); // '>'もシアンで表示
                        result.push(c);
                        result.push_str(RESET); // 色をリセット
                        in_brackets = false; // ブラケット外に出たフラグをリセット
                    }
                    '"' => {
                        if in_quotes {
                            result.push(c); // クオートの終わり
                            result.push_str(RESET); // クオートの終了時にリセット
                        } else {
                            result.push_str(GREEN); // クオートの開始時に緑色にする
                            result.push(c);
                        }
                        in_quotes = !in_quotes; // クオート状態をトグル
                    }
                    ' ' => {
                        if in_brackets && first_word {
                            result.push_str(RESET); // 最初の単語全体が終わったらリセット
                            first_word = false; // 最初の単語フラグをリセット
                        }
                        result.push(c); // スペースをそのまま追加
                    }
                    _ => {
                        if in_brackets && !in_quotes {
                            if first_word {
                                result.push_str(BLUE); // 最初の単語は青色
                            } else {
                                result.push_str(YELLOW); // それ以降は黄色
                            }
                        } else if in_quotes {
                            result.push_str(GREEN); // クオート内は常に緑
                        }
                        result.push(c); // 通常の文字を追加
                    }
                }
            }

            result // 結果を返す
        }
    };
}
