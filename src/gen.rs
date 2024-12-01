use anyhow::bail;

pub fn year(y: u64) -> anyhow::Result<()> {
    if y < 2015 || y > 2030 {
        bail!("unexpected year {}", y);
    }
    println!("gen {}", y);

    Ok(())
}
