use anyhow::{anyhow, Result};
use hyper::{service::Service, Body, Client, Request, StatusCode};
use hyper_tls::HttpsConnector;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
// target_arch: "x86" "x86_64" "mips""powerpc" "powerpc64" "arm" "aarch64"
// target_os:"windows""macos""ios""linux""android""freebsd""dragonfly""openbsd""netbsd"

static IPFS_BASE_URL: &'static str = "https://github.com";

/// version: v0.15.0
pub async fn download(version: &str, install_path: &str) -> Result<()> {
    //
    let platform = get_target_os();
    let arch = get_target_arch();

    let url = get_download_url(version, platform, arch, IPFS_BASE_URL)?;

    tracing::debug!("download_url:{}", url);

    let content_bytes = get(url.as_str()).await?;

    tracing::debug!("content_bytes length:{}", content_bytes.len());

    if platform == "windows" {
        write_to_file(install_path, "ipfs.zip", &content_bytes[..].to_vec())?;
        //解压
    } else {
        write_to_file(install_path, "ipfs.tar.gz", &content_bytes[..].to_vec())?;
        //解压
    }

    Ok(())
}

pub fn write_to_file(path: &str, filename: &str, content: &Vec<u8>) -> Result<()> {
    //
    let storage_path = PathBuf::from(path);
    std::fs::create_dir_all(storage_path.as_path())?;

    let filename = storage_path.join(filename);
    //创建文件
    let mut output = File::create(filename.as_path())?;
    output.write_all(content)?;
    Ok(())
}

fn get_download_url(version: &str, platform: &str, arch: &str, dist_url: &str) -> Result<String> {
    if platform == "" {
        return Err(anyhow!("不支持的platform!"));
    }

    if arch == "" {
        return Err(anyhow!("不支持的arch!"));
    }

    if platform == "windows" {
        Ok(format!(
            "{}/ipfs/kubo/releases/download/{}/kubo_{}_{}-{}.zip",
            dist_url, version, version, platform, arch
        ))
    } else {
        Ok(format!(
            "{}/ipfs/kubo/releases/download/{}/kubo_{}_{}-{}.tar.gz",
            dist_url, version, version, platform, arch
        ))
    }
}

fn get_target_arch() -> &'static str {
    let target_arch_str = if cfg!(target_arch = "x86") {
        "386"
    } else if cfg!(target_arch = "x86_64") {
        "amd64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "unknown"
    };
    target_arch_str
}

fn get_target_os() -> &'static str {
    let target_os_str = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "ios") {
        ""
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "android") {
        ""
    } else if cfg!(target_os = "freebsd") {
        "freebsd"
    } else if cfg!(target_os = "dragonfly") {
        "dragonfly"
    } else if cfg!(target_os = "openbsd") {
        "openbsd"
    } else if cfg!(target_os = "netbsd") {
        ""
    } else {
        ""
    };
    target_os_str
}

/// 内部请求的方法
async fn get(path: &str) -> Result<bytes::Bytes> {
    //请求地址
    let uri = path.parse::<http::Uri>()?;

    let https = HttpsConnector::new();
    let mut client = Client::builder().build::<_, hyper::Body>(https);
    // let mut client = Client::new();
    let body = Body::empty();

    let request = Request::get(uri)
        .header("user-agent", "IDNS/1.0")
        .body(body)?;
    //发送请求
    let resp = client.call(request).await?;

    //获取版本的信息resp.headers()
    tracing::debug!("Response: {:?}", resp.status());
    let headers = resp.headers();
    //location
    if headers.contains_key("location") {
        let location = headers.get("location").unwrap().to_str().unwrap();

        tracing::debug!("download_file_url: {:?}", location);

        let body = Body::empty();
        let request = Request::get(location)
            .header("user-agent", "IDNS/1.0")
            .body(body)?;
        //
        let resp = client.call(request).await?;

        //
        let result = hyper::body::to_bytes(resp).await;
        match result {
            Ok(plain) => {
                return Ok(plain);
            }
            Err(err) => {
                return Err(anyhow!("Fail request {}!", err));
            }
        }
    }
    Err(anyhow!("Fail request!"))
}
