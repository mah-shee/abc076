#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    let s_len = s.len();
    let t_len = t.len();
    if s_len < t_len {
        println!("UNRESTORABLE");
        return;
    }
    // sの中にtを含むことができるか判別する
    // 複数の位置にsを含むことができる場合、一番うしろの位置にtを含むことで辞書順最小
    // つまりsの最後尾から走査し、tを含むことができるか判別し残りの?はaで埋める
    let mut ans = false;
    let mut index = 0;
    for i in (0..=s_len - t_len).rev() {
        let mut f = true;
        for j in 0..t_len {
            // 最後尾から一文字ずつtと一致もしくは?かどうか走査する
            if t[j] != s[i + j] && s[i + j] != '?' {
                // tと一致しないまたは?じゃないときこのiの位置でtの挿入は無理なのでbreakする
                f = false;
                break;
            }
        }
        if f {
            // tの長さ分一致したとき、tはsの中に含まれるので挿入する
            ans = true;
            index = i;
            break;
        }
    }
    if ans {
        for k in 0..t_len {
            s[index + k] = t[k];
        }
        for i in 0..s_len {
            if s[i] == '?' {
                s[i] = 'a';
            }
            print!("{}", s[i]);
        }
    } else {
        println!("UNRESTORABLE");
    }
}
