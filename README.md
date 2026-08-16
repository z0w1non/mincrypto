# mincrypto

## 概要
暗号理論に関連する数学の初歩的なロジックの実装

## 技術的要素
* 試し割り法による因数分解
* ユークリッド互除法により最大公約数を算出
* 最小公倍数を算出
* 拡張ユークリッド互除法により不定方程式の特殊解を算出
* 繰り返し二乗法によりべき乗を算出(O(logN))

## 実行方法
```bash
cargo test
```

## 実行結果(例)
```bash
PS C:\mincrypto> cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src\main.rs (target\debug\deps\mincrypto-d21d375786af82b7.exe)

running 11 tests
test test::test_extended_gcd_1 ... ok
test test::test_extended_gcd_2 ... ok
test test::test_gcd_1 ... ok
test test::test_lcm_2 ... ok
test test::test_gcd_2 ... ok
test test::test_extended_gcd_4 ... ok
test test::test_pow ... ok
test test::test_factorization_2 ... ok
test test::test_factorization_1 ... ok
test test::test_lcm_1 ... ok
test test::test_extended_gcd_3 ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## TODO
* (k, n)に対するシャミアの秘密分散
* ラグランジュ補間
* フェルマーの小定理による乗法逆元の算出