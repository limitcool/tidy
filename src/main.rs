mod versions;
#[tokio::main]
async fn main() {
    let files = get_files_recursive("./src".to_string());
    let mut crates = vec![];

    for f in files {
        let re = regex::Regex::new(r#"use\s(\w*).*"#).unwrap();
        let content = std::fs::read_to_string(f) // 使用 fs 的读取文件函数
            .expect("Something went wrong reading the file");
        for cap in re.captures_iter(content.clone().as_str()) {
            crates.push(cap[1].to_string());
        }
    }
    // 去除crates中的重复元素
    crates.retain(|x| x != "std" && x != "core" && x != "crate");
    crates.sort();
    crates.dedup();
    if crates.len() == 0 {
        println!("no crate found");
    } else {
        println!("{:?}", crates);
        let mut map = std::collections::HashMap::new();
        for create in crates {
            let r = get_crate_version(create.to_string()).await;
            map.insert(create, r);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        println!("{:?}", map);
    }
}

pub fn get_files(path: String) -> Vec<String> {
    // vec.drain_filter drain的意思是 排出 的意思，所以这个函数就是排出过滤器，接收一个回调函数，然后把回调函数里面返回true的元素就会排出，自然也就从原本的vec里面删除掉了。然后有需要的话还可以搜集排出的元素。
    // 读取当前目录下的所有文件
    let paths = std::fs::read_dir(path).unwrap();
    let mut files: Vec<String> = Vec::new();
    for path in paths {
        files.push(path.unwrap().path().display().to_string());
    }
    return files;
}

// 递归获取指定目录下的所有文件
pub fn get_files_recursive(path: String) -> Vec<String> {
    let paths = std::fs::read_dir(path).unwrap();
    let mut files: Vec<String> = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            files.append(&mut get_files_recursive(path.display().to_string()));
        } else {
            files.push(path.display().to_string());
        }
    }
    return files;
}

pub async fn get_crate_version(name: String) -> String {
    let url = format!("https://crates.io/api/v1/crates/{}/versions", name);
    // 设置Ua为 : "tidy github.com/limitcool"
    let client = reqwest::Client::builder()
        .user_agent("tidy github.com/limitcool/tidy")
        .build()
        .unwrap();

    let j: serde_json::Value = client.get(url).send().await.unwrap().json().await.unwrap();
    // println!("{}", result);
    println!("{:?}", j["versions"][0]["num"]);
    return j["versions"][0]["num"].to_string();
}
