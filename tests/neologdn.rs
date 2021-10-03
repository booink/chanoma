use chanoma::Chanoma;

#[test]
fn noop() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("abc"), "abc");
    assert_eq!(chanoma.normalize_with_positions("abc").text(), "abc");
}

#[test]
fn digits() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("０１２３４５６７８９"), "0123456789");
    assert_eq!(chanoma.normalize_with_positions("０１２３４５６７８９").text(), "0123456789");
}

#[test]
fn upper_alphabets() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    assert_eq!(chanoma.normalize_with_positions("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ").text(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
}

#[test]
fn lower_alphabets() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), "abcdefghijklmnopqrstuvwxyz");
    assert_eq!(chanoma.normalize_with_positions("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ").text(), "abcdefghijklmnopqrstuvwxyz");
}

#[test]
fn punct() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}");
    assert_eq!(chanoma.normalize_with_positions("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝").text(), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}");
}

#[test]
fn wide_punct() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("＝。、・「」"), "＝。、・「」");
    assert_eq!(chanoma.normalize_with_positions("＝。、・「」").text(), "＝。、・「」");
}

#[test]
fn half_width_kana() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("ﾊﾝｶｸ"), "ハンカク");
    assert_eq!(chanoma.normalize_with_positions("ﾊﾝｶｸ").text(), "ハンカク");
}

#[test]
fn hyphen() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("o₋o"), "o-o");
    assert_eq!(chanoma.normalize_with_positions("o₋o").text(), "o-o");
}

#[test]
fn choonpu() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("majika━"), "majikaー");
    assert_eq!(chanoma.normalize_with_positions("majika━").text(), "majikaー");
}

#[test]
fn remove_tildes() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("わ〰い"), "わい");
    assert_eq!(chanoma.normalize_with_positions("わ〰い").text(), "わい");
}

#[test]
fn consecutive_choonpus() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("スーパーーーー"), "スーパー");
    assert_eq!(chanoma.normalize_with_positions("スーパーーーー").text(), "スーパー");
}

#[test]
fn hoge1() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("!#"), "!#");
    assert_eq!(chanoma.normalize_with_positions("!#").text(), "!#");
}

#[test]
fn remove_wide_spaces() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("ゼンカク　スペース"), "ゼンカクスペース");
    assert_eq!(chanoma.normalize("お             お"), "おお");
    assert_eq!(chanoma.normalize("      おお"), "おお");
    assert_eq!(chanoma.normalize("おお      "), "おお");

    assert_eq!(chanoma.normalize_with_positions("ゼンカク　スペース").text(), "ゼンカクスペース");
    assert_eq!(chanoma.normalize_with_positions("お             お").text(), "おお");
    assert_eq!(chanoma.normalize_with_positions("      おお").text(), "おお");
    assert_eq!(chanoma.normalize_with_positions("おお      ").text(), "おお");
}

#[test]
fn remove_spaces() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize_with_positions("検索 エンジン 自作 入門 を 買い ました!!!").text(), "検索エンジン自作入門を買いました!!!");
    assert_eq!(chanoma.normalize_with_positions("アルゴリズム C").text(), "アルゴリズムC");
    assert_eq!(chanoma.normalize_with_positions("　　　ＰＲＭＬ　　副　読　本　　　").text(), "PRML副読本");
}

#[test]
fn keep_spaces() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("Coding the Matrix"), "Coding the Matrix");
    assert_eq!(chanoma.normalize_with_positions("Coding the Matrix").text(), "Coding the Matrix");
}

#[test]
fn mixed_spaces() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"), "南アルプスの天然水Sparking Lemonレモン一絞り");
    assert_eq!(chanoma.normalize("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り"), "南アルプスの天然水-Sparking*Lemon+レモン一絞り");

    assert_eq!(chanoma.normalize_with_positions("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り").text(), "南アルプスの天然水Sparking Lemonレモン一絞り");
    assert_eq!(chanoma.normalize_with_positions("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り").text(), "南アルプスの天然水-Sparking*Lemon+レモン一絞り");
}

#[test]
fn ligature() {
    let chanoma = Chanoma::neologdn();
    assert_eq!(chanoma.normalize("あは゛れる　アハ゜ホテル"), "あばれるアパホテル");
    assert_eq!(chanoma.normalize_with_positions("あは゛れる　アハ゜ホテル").text(), "あばれるアパホテル");
}
