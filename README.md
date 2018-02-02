# Memo

Memo is a simple tool written in Rust that keeps a map of strings in a config folder for easy access from the terminal.  
  
For example:  
```bash
$ memo -s key value
$ memo key # prints value
```

This can also be used with files and paths by using the -f flag.  
```bash
$ memo -sf example ~
$ memo example # prints the full path to the home directory
```

Installation currently requires [cargo][cargo-install]. To install, clone the repository and run `cargo install` in the top level directory.  

I recommend putting the following in your bashrc so that you can jump to folders that are set in memo using the jmp command.  
```bash
function jmp {
    DIR=$(memo $1)
    if [[ $? == 0 ]];
    then
        cd $DIR
    else
        echo $DIR
    fi
}
```

On installation, run `memo -h` for the list of possible arguments.

[cargo-install]: https://doc.rust-lang.org/cargo/getting-started/installation.html
