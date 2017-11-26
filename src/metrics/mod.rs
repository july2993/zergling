use prometheus::*;


lazy_static! {
    pub static ref HTTP_REQ_COUNTER_VEC: CounterVec = 
        register_counter_vec!(
            "zergling_req_total",
            "the number of http request",
            &["type", "method"]
            ).unwrap();


    pub static ref HTTP_REQ_HISTOGRAM_VEC: HistogramVec = register_histogram_vec!(
            "zergling_http_req_duration_seconds",
            "The HTTP request latencies in seconds.",
            &["type", "method"]
            ).unwrap();

}
