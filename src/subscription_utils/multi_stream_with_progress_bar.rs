use crate::subscription_utils::iced_send::IcedSend;
use futures::{StreamExt, stream};
use futures_channel::mpsc::Sender;
use rayon::prelude::*;
use std::iter;

pub async fn multi_stream_with_progress_bar<D, RD, RC, M, P>(
    data: &[D],
    load_threads: usize,
    sender: Option<Sender<M>>,
    add_progress_message: impl Fn(f32) -> P + Clone,
    handler: impl Fn(&D) -> Vec<RD> + Sync + Send + Clone,
) -> RC
where
    D: Sync + Clone,
    RD: Send + Clone,
    RC: FromIterator<RD>,
    M: From<P>,
{
    let data = data
        .chunks(load_threads)
        .map(From::from)
        .collect::<Vec<Vec<_>>>();
    let count = data.len();

    stream::iter(
        data.iter()
            .zip(iter::repeat(handler))
            .map(|(datas, handler)| async {
                let result = datas.par_iter().flat_map(handler).collect::<Vec<_>>();
                sender
                    .clone()
                    .send_add_progress(count, add_progress_message.clone())
                    .await;
                result
            }),
    )
    .then(|task| task)
    .collect::<Vec<_>>()
    .await
    .iter()
    .flatten()
    .cloned()
    .collect::<RC>()
}
