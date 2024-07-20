use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // 写入字字节符串的一些前缀, 但不一定是全部
    let n = file.write(b"some bytes").await?;

    println!("Write the first {} bytes of 'some bytes'.", n);
    Ok(())
}
