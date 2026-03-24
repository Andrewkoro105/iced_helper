use futures_channel::mpsc::Sender;
use std::time::Duration;
use tokio::time::sleep;

pub trait IcedSend<M, P> {
    fn send(&mut self, message: M) -> impl Future<Output = ()>;
    fn send_add_progress(&mut self, count: usize, add_progress_message: impl Fn(f32) -> P) -> impl Future<Output = ()>;
}

impl<M: From<P>, P> IcedSend<M, P> for Sender<M> {
    async fn send(&mut self, message: M) {
        <Self as futures::SinkExt<_>>::send(self, message)
            .await
            .unwrap();
        sleep(Duration::from_millis(1)).await;
    }

    async fn send_add_progress(&mut self, count: usize, add_progress_message: impl Fn(f32) -> P) {
        self.send(add_progress_message(1. / count as f32).into())
            .await;
    }
}

impl<M: From<P>, P> IcedSend<M, P> for Option<Sender<M>> {
    async fn send(&mut self, message: M) {
        if let Some(sender) = self.as_mut() {
            sender.send(message).await;
        }
    }

    async fn send_add_progress(&mut self, count: usize, add_progress_message: impl Fn(f32) -> P) {
        if let Some(sender) = self.as_mut() {
            sender.send_add_progress(count, add_progress_message).await;
        }
    }
}
