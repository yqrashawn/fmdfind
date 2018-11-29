extern crate regex;
extern crate shellexpand;

use regex::RegexSet;
use std::process::Command;

fn mdfind(onlyin: &str, query: &str) {
    let output = Command::new("/usr/bin/mdfind")
        .arg("-onlyin")
        .arg(shellexpand::full(onlyin).unwrap().to_mut())
        .arg(query)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // println!("stdout is: {}", stdout);
    let re = RegexSet::new(&[
        r"node_modules",
        r"Application Support",
        r"Library.*Logs",
        r"Library.*cache",
        r"Library/Containers",
        r"Group Containers",
        r"Users/Rashawn/emacs",
        r"Library",
        // r"Library/Mockplus2",
        // r"Library/Metadata",
        // r"Library/VirtualBox",
        r"THIRD",
        // r"THIRD/reamcs",
        // r"THIRD/node",
        // r"THIRD/boodle",
        // r"THIRD/spacemacs",
        // r"THIRD/neovim",
        r"min.*",
    ])
    .unwrap();

    for (_num, line) in stdout.lines().enumerate() {
        let l = line;
        let matches: Vec<_> = re.matches(&l).into_iter().collect();
        if matches.len() == 0 {
            println!("{}", l);
        }
    }
}

fn main() {
    mdfind(
        "~/",
        "kMDItemContentTypeTree = 'public.text' && kMDItemFSContentChangeDate > $time.today(-1)",
    );
    mdfind("~/", "kMDItemContentTypeTree = 'public.text' && kMDItemFSContentChangeDate > $time.today(-4) && kMDItemFSContentChangeDate <= $time.today(-1)");
    mdfind("~/", "kMDItemContentTypeTree = 'public.text' && kMDItemFSContentChangeDate > $time.today(-14) && kMDItemFSContentChangeDate <= $time.today(-4)");
    mdfind("~/", "kMDItemContentTypeTree = 'public.text' && kMDItemFSContentChangeDate > $time.today(-30) && kMDItemFSContentChangeDate <= $time.today(-14)");
}
