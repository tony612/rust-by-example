use anyhow::Result;
use tokio::sync::watch;

async fn watch() -> Result<()> {
    let (tx, mut rx) = watch::channel("hello");
    let mut rx2 = rx.clone();

    let join_handle = tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("received 1 = {:?}", *rx.borrow());
            return;
        }
    });

    let join_handle2 = tokio::spawn(async move {
        while rx2.changed().await.is_ok() {
            println!("received 2 = {:?}", *rx2.borrow());
            return;
        }
    });

    tx.send("world")?;
    join_handle.await?;
    join_handle2.await?;
    Ok(())
}

#[tokio::test]
async fn test_watch() {
    watch().await.unwrap();
    assert!(true);
}
