use regex::Regex;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};
use url::Url;

fn main() {
    // REGEX
    // rungameid\/(\d*)\n
    // games\\(\w*).ico

    println!("Hello, world!");

    let reg_gameid = Regex::new(r"rungameid\/(\d*)").unwrap();
    let reg_iconid = Regex::new(r"games\\(\w*).ico").unwrap();

    let desktop_path: &Path = Path::new("/data");
    let desktop_files = read_dir(desktop_path).unwrap();

    let filtered_files =
        desktop_files.filter(|file| file.as_ref().unwrap().path().extension().unwrap() == "url");

    // https://cdn.cloudflare.steamstatic.com/steamcommunity/public/images/apps/892970/e63a5c06f103cef62c44a492c48af7d0118c9421.ico

    let steam_data = filtered_files.into_iter().map(|url| {
        let content = read_to_string(url.as_ref().unwrap().path()).unwrap();
        let gameid = reg_gameid
            .captures(&content)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let iconid = reg_iconid
            .captures(&content)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let mut icon_url = Url::parse("")
            .unwrap()
            .join(gameid)
            .unwrap()
            .join(iconid)
            .unwrap();

        println!("file {}", url.as_ref().unwrap().path().display());
        println!("gameid {}", gameid);
        println!("iconid {}", iconid);
        println!("icon url {:?}", icon_url.as_str());
        println!();
        println!();

        (url, content);
    });

    for a in steam_data {
        println!("{:?}", a);
    }

    // for path in filtered_files {
    //     let rts = read_to_string(path.as_ref().unwrap().path());

    //     println!("{:?}", rts);
    //     // println!("{:?}", path.unwrap().path());
    //     println!("File {}", path.unwrap().path().display());
    // }

    // // Read desktop files - find lnk files
    // for path in desktop_files {
    //     // escape if not a lnk
    //     if path.as_ref().unwrap().path().extension().unwrap() != "lnk" {
    //         continue;
    //     }
    // }
}
