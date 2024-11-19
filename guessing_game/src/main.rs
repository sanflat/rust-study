//************/
// let、match、メソッド、関連関数、外部クレーとの使い方などをちょっと触ってみるハンズオン 
//************/
// ユーザーの入力を受け付けるライブラリ：ioライブラリはstdと呼ばれる標準のライブラリになる
// ユーザーの入力を受け付けるライブラリ：Rngライブラリはrandと呼ばれる外部のライブラリになり、Cargo.tomlに依存関係を記載し、cargo buildでレジストリから取得する必要がある
// stdからOrderingと呼ばれるenumの型を導入している
// ☆cargo doc --openコマンドを使うと、すべての依存ライブラリ（クレート）が提供するドキュメントをローカルでビルドして、ブラウザで開いてくれる
// use を使ってインポートする
// prelude（プレリュード）と呼ばれる、自動的にインポートするものもある。
// preludeの詳細：https://doc.rust-lang.org/std/prelude/index.html
use std::io; 
use rand::Rng;
use std::cmp::Ordering;

// main関数の実行はrootで、cargo runコマンドを実行でOK
// main関数がエントリーポイント（スタート地点）
// fn構文は関数を新しく宣言するもの（）は引数部分。{}は関数の本体となる。
fn main() {
    // println!は画面に文字列を表示するマクロ
    println!("Guess the number!");

    // 乱数を生成する。範囲式は（開始..終了）の形式で記載する。下限値は含むが上限値は含まないので、1から100までの数をリクエストしたい場合以下のような書き方になる
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // 値を変数に保持する
        // 変数はletで宣言する
        // Rustでは変数はデフォルトで不変(immutable)になる
        // 下記のguess は可変(mutable)であって欲しいため、let mut 変数名と宣言している
        // 等号記号( = )はRustに、今この変数を何かに束縛したいことを伝える
        // 等号記号の右側の値に束縛される。下記コードであればStringの空のインスタンスに束縛されている
        // ※Stringライブラリはインポートしていないが、これが使えるのはプレリュードであるから
        // ::ner の行にある::構文はnewがString型の関連関数であることを示している
        // 関連関数とは、ある型(この場合はString)に対して実装される関数のこと
        let mut guess = String::new();  // 可変変数をguessは、String型の空のインスタンスに束縛されている
    
        // ユーザーの入力を受け取る
        // ioモジュールのstdin関数を呼び出して、ユーザー入力を処理できるようにする
        // 事前にuseでライブラリをインポートしてない場合は、use io::stdin()で呼び出せる
        // read_lineでは、引数として可変変数を渡し、どの行に文字列を格納するかを指示している
        // read_lineが文字列を変更できるように、可変である必要がある
        // &mut guess の＆は。この引数が参照であることを示している
        // 参照している(いわば、新しくメモリにプール領域を使わず、スタックから示しているだけ)ため、そのデータを何度もメモリにコピーしなくて済む
        // ☆参照は複雑な機能（訳注：一部のプログラム言語では正しく使うのが難しい機能）ですが、Rustの大きな利点の一つは参照を安全かつ簡単に使用できる
        // read_lineメソッドは、Result型を返しています。Result型は列挙型(enum)になっている
        // Resultの列挙子はOk（処理が成功）かErr（処理が失敗）で、Okの中には正常に生成された値が入り、Errは処理が失敗した過程や理由についての情報が含まれる
        // Result型のインスタンスには、expectメソッドがあり、ResultがErrの場合、プログラムをクラッシュさせ、引数として渡したメッセージを表示する
        io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");
    
        // guessのシャドーイング。数値型のguess変数を作成
        // シャドーイングはある型から別の型に変換する時によく使われる
        // trim()はStringのメソッドで文字列の先頭と末尾の空白をすべて削除する、また\nや\r\nをの改行文字を削除する
        // parseメソッドは文字列をパースしてなんらかの数値にする、様々な数値型に変換できるので、let guess: u32と注釈をつける必要がある
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num, // ParseがResult型を返しOk値に格納したnumの値を返す
            Err(_) => continue, // Err(_)の場合は、continueとする アンスコはすべての値を受け入れる
        };
    
        // {}を使うことで、その位置に値を保持する事ができる
        //　複数値を保持したい場合は、println!("guess: {} / {}", one, two)のようになる
        println!("You gueese: {}", guess);
    
        // cmpメソッドは二つの値（guessとsecret_number）の比較を行っている
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // breakでループ処理を抜ける
            }
        }        
    }

}
