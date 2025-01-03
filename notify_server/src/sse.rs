use axum::response::sse::Event;
use axum::response::Sse;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use futures::stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::{Stream, StreamExt};

pub(crate) async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
