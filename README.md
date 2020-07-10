# mysh 2
my shell implementation 2 on Rust

usage:
```
$ cargo run
```

## features
#### command execution
```sh
$ echo hoge
hoge
```

#### pipe
```sh
$ echo hogehoge | sed s/hoge/fuga/ | rev
egohaguf
```

#### redirection
```sh
$ ls > test.txt
$ echo hello >> test.txt
$ cat test.txt
Cargo.lock
Cargo.toml
README.md
src
test.txt
hello
```

#### read from file
```sh
$ tr . _ < Cargo.lock
# This file is automatically @generated by Cargo_
# It is not intended for manual editing_
[[package]]
name = "mysh2"
version = "0_1_0"
```

#### here document
```sh
$ grep hello << EOF
hello
bye
hello world!
EOF
hello
hello world!
```

#### here strings
```sh
$ sed s/$/world/ <<< hello
helloworld
```
