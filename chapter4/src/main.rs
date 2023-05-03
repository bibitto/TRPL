fn main() {
    let mut s = String::from("hello"); // Strin型は可変故にヒープ領域でallocateされる
    s.push_str(", world!"); // 一方で文字数リテラルは普遍なのでコンパイル時に長さがわかる（stack）
    println!("{}", s);

    // 以下の二つの変数x, yの値はスタックに積まれる
    let x = 5;
    let y = x;

    // Stringバージョンはどうなる？
    let s1 = String::from("hello");
    let s2 = s1; // s1とs2のヒープ上のデータは同じものを参照している？
    println!("{}, world!", s2); // そこでs1, s2がそれぞれdropしようとして起こる２重解放を防ぐためにs1からs2に所有権がムーブする
    // s1は無効になっているためs1を出力しようとするとエラーになる

    // cloneを使えばdeep copyもできる
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // あれ、最初にやった整数の場合はなんでcloneとかしてないのにできたんだっけ？つまりムーブされてない
    // 理由は整数値はstack領域にすっぽり収まるから → xの所有権をムーブする必要性がない
    // 一般的に、単純なスカラー値およびその集合は単なるコピー
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    slice_lesson()
}

fn slice_lesson() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // wordの中身は、値5になる
    s.clear(); // Stringを空にする。つまり、""と等しくする
    // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
    // wordは今や完全に無効だが、コンパイルを通り抜ける
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // String to Bytes Arrow
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    // 空白がなければ全ての値を返す
    &s[..]
}