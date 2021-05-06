use super::*;

pub fn valid_content_ipfs() -> Content {
    Content::IPFS(b"QmRAQB6YaCaidP37UdDnjFY5aQuiBrbqdyoW1CaDgwxkD4".to_vec())
}

pub fn invalid_content_ipfs() -> Content {
    Content::IPFS(b"QmRAQB6DaazhR8".to_vec())
}
