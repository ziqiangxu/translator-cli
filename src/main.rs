use reqwest;
use std::env;
use std::collections::HashMap;

fn query(text: &str) -> Result<HashMap<String, String>, reqwest::Error> {
    // todo update the url, and encode it
    let url = format!("http://translate.ziqiangxu.xyz:8000/translate?q={}&from=en&to=zh", text);
    let body = reqwest::blocking::get(&url)?
                           .json::<HashMap<String, String>>()?;
    Ok(body)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Todo check arguments
    let text = &args[1].to_string();
    let res = query(text);
    match res {
        Ok(r) => {
            // Formate the result
            let explain = r.get("explain");
            let line = format!("原文：{}\n------\n", text);
            let mut out = String::from(&line);
            match explain {
                None => None,
                Some(s) => {
                    let line = format!("解释：{}\n", s);
                    out.push_str(&line);
                    Some(s)
                },
            };

            let translate = r.get("translation");
            match translate {
                None => None,
                Some(s) => {
                    let line = format!("翻译：{}\n", s);
                    out.push_str(&line);
                    Some(s)
                }
            };

            println!("{}", &out);
            // println!("Result got: {:#?}", r);
        },
        Err(e) => panic!("Failed to get translation: {:?}", e)
    };
}
