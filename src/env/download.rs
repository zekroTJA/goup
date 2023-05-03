use crate::versions::Version;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub fn get_download_url(v: &Version) -> String {
    format!("https://go.dev/dl/go{v}.linux-amd64.tar.gz")
}

#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub fn get_download_url(v: &Version) -> String {
    format!("https://go.dev/dl/go{v}.linux-386.tar.gz")
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub fn get_download_url(v: &Version) -> String {
    format!("https://go.dev/dl/go{v}.linux-arm64.tar.gz")
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub fn get_download_url(v: &Version) -> String {
    format!("https://go.dev/dl/go{v}.darwin-amd64.tar.gz")
}

#[cfg(all(target_os = "macos", target_arch = "x86"))]
pub fn get_download_url(v: &Version) -> String {
    format!("https://go.dev/dl/go{v}.darwin-386.tar.gz")
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub fn get_download_url(v: &Version) -> String {
    format!("https://go.dev/dl/go{v}.darwin-arm64.tar.gz")
}
