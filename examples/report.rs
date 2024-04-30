#[macro_use]
extern crate trackable;

use cf_rustracing::sampler::AllSampler;
use cf_rustracing::tag::Tag;
use rustracing_jaeger::reporter::JaegerCompactReporter;
use rustracing_jaeger::Tracer;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> trackable::result::MainResult {
    let (tracer, mut span_rx) = Tracer::new(AllSampler);
    {
        let span0 = tracer.span("main").start();
        thread::sleep(Duration::from_millis(10));
        {
            let mut span1 = tracer
                .span("sub")
                .child_of(&span0)
                .tag(Tag::new("foo", "bar"))
                .start();
            span1.log(|log| {
                log.error().message("something wrong");
            });
            thread::sleep(Duration::from_millis(10));
        }
    }

    let mut reporter = track!(JaegerCompactReporter::new("example"))?;
    reporter.add_service_tag(Tag::new("hello", "world"));

    let mut spans = vec![];

    span_rx.recv_many(&mut spans, 10).await;
    track!(reporter.report(&spans))?;
    
    Ok(())
}
