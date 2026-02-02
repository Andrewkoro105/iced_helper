use crate::subscription_utils::iced_send::IcedSend;
use crate::unique_number;
use futures_channel::mpsc::Sender;
use iced_futures::{Subscription, stream};
use std::hash::Hash;
use std::time::Instant;
use tracing::info;

pub trait IcedUiModule<RM, R, Settings> {
    fn start(&self) -> bool;

    fn result_subscription(&self, settings: &Settings) -> Option<Subscription<RM>>;
}

pub trait UniqueNumber {
    fn unique_number(&self) -> impl Hash + 'static {
        unique_number!()
    }
}

pub trait IcedMessage<R>: Sized {
    const SET_ZERO_PROGRESS: Self;
    const SET_RESULT_DATA: fn(Box<R>) -> Self;
    const STOP: Self;
}

pub trait IcedSubscriptionData<M, R, Settings> {
    fn run(
        self,
        settings: &Settings,
        sender: Option<Sender<M>>,
    ) -> impl std::future::Future<Output = R> + std::marker::Send;
}

pub trait BaseSubscription<R, M, RM, D, Settings> {
    fn subscription(&self, settings: &Settings) -> Subscription<M>;
}

pub trait CoreSubscription<R, M, D, Settings> {
    fn subscription_core(&self, settings: &Settings) -> Subscription<M>;
}

impl<IM, R, M, RM, D, Settings> BaseSubscription<R, M, RM, D, Settings> for IM
where
    IM: IcedUiModule<RM, R, Settings> + UniqueNumber,
    M: IcedMessage<R> + From<RM> + Send + 'static,
    RM: 'static,
    D: IcedSubscriptionData<M, R, Settings> + for<'a> From<&'a IM> + Send + 'static,
    R: Send,
    Settings: Clone + Send + 'static,
{
    fn subscription(&self, settings: &Settings) -> Subscription<M> {
        if self.start() {
            <IM as CoreSubscription<R, M, D, Settings>>::subscription_core(self, settings)
        } else if let Some(subscription) = self.result_subscription(settings) {
            subscription.map(Into::into)
        } else {
            Subscription::none()
        }
    }
}

impl<IM, R, M, D, Settings> CoreSubscription<R, M, D, Settings> for IM
where
    IM: UniqueNumber,
    M: IcedMessage<R> + Send + 'static,
    D: IcedSubscriptionData<M, R, Settings> + for<'a> From<&'a IM> + Send + 'static,
    R: Send,
    Settings: Clone + Send + 'static,
{
    fn subscription_core(&self, settings: &Settings) -> Subscription<M> {
        let data: D = self.into();
        let settings = settings.clone();
        Subscription::run_with_id(
            self.unique_number(),
            stream::channel(100, move |mut sender| async move {
                let start = Instant::now();
                sender.send(M::SET_ZERO_PROGRESS).await;
                sender
                    .send(M::SET_RESULT_DATA(Box::new(
                        data.run(&settings, Some(sender.clone())).await,
                    )))
                    .await;
                info!("find faces finished");
                sender.send(M::STOP).await;

                info!("Время выполнения: {:?}", start.elapsed());
            }),
        )
    }
}
