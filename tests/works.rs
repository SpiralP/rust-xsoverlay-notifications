use xsoverlay_notifications::{Error, Notification, XSOverlayNotifier};

#[tokio::test]
async fn works() -> Result<(), Error> {
    let notifier = XSOverlayNotifier::new().await?;

    notifier
        .send(&Notification {
            ..Default::default()
        })
        .await?;

    Ok(())
}
