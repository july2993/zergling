[![Status](https://travis-ci.org/july2993/zergling.svg?branch=master)](https://travis-ci.org/july2993/zergling)

seaweedfs re-implemented in Rust.

## Why?

To learn Rust.

## Prerequisites

Rust nightly.  use 1.23.0-nightly (827cb0d61 2017-11-26) to develop now.

## Features

- Can choose no replication or different replication level, rack and data center aware
- ~~Automatic master servers failover - no single point of failure (SPOF)~~ only suppose single master now
- ~~Automatic compaction to reclaimed disk spaces after deletion or update~~ TODO
- Support Etag, ~~Accept-Range~~, Last-Modified, custom header pairs start with "Zergling-".
- Support only the in-memory mode for needle map.
- ~~Chunking large files~~



## Example Usage

IMPORTANT: some important feature like compaction not implemented yet,  more test and optimization needed, so don't use it unless you just want to hack on it!

### start master server

default it will listen on port 9333 to serve HTTP and port 9334(http port +1) to serve grpc( to communicate with volume server) 

```
➜  release git:(develop) ✗ ./zergling master
starting master server[9333]....

```

### start volume server

```
➜  release git:(develop) ✗ ./zergling volume --port 8080 --dir ./vdata:70
starting volumn server....

```

write or read file like seaweeds.

assign fid

```
➜  sh git:(develop) ✗ curl http://localhost:9333/dir/assign
{"fid":"3,9ecf2125e547","url":"127.0.0.1:8080","publicUrl":"127.0.0.1:8080","count":1,"error":""}%
```

upload file

```
sh git:(develop) ✗ curl   -F file=@./durtation.png http://127.0.0.1:8080/3,9ecf2125e547
{"name":"durtation.png","size":121846,"error":""}%
```

to delete

```
curl -X DELETE http://127.0.0.1:8080/3,9ecf2125e547
```

to read file like seaweeds but no  scaled version of an image supported.



## Benchmark

since, it has the same api, you can use seaweedfs tool to benmark

```
➜  weed git:(master) ✗ ./weed benchmark -n 10000
This is SeaweedFS version 0.76 linux amd64

------------ Writing Benchmark ----------
Completed 3479 of 10000 requests, 34.8% 3447.0/s 3.5MB/s
Completed 6773 of 10000 requests, 67.7% 3324.6/s 3.3MB/s

Concurrency Level:      16
Time taken for tests:   2.903 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      10555292 bytes
Requests per second:    3444.68 [#/sec]
Transfer rate:          3550.74 [Kbytes/sec]

Connection Times (ms)
              min      avg        max      std
Total:        0.4      4.3       50.3      3.2

Percentage of the requests served within a certain time (ms)
   50%      3.5 ms
   66%      4.3 ms
   75%      5.0 ms
   80%      5.5 ms
   90%      7.2 ms
   95%      9.4 ms
   98%     14.0 ms
   99%     18.8 ms
  100%     50.3 ms

------------ Randomly Reading Benchmark ----------

Concurrency Level:      16
Time taken for tests:   0.792 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      0 bytes
Requests per second:    12619.81 [#/sec]
Transfer rate:          0.00 [Kbytes/sec]

Connection Times (ms)
              min      avg        max      std
Total:        0.0      1.0       13.7      0.9

Percentage of the requests served within a certain time (ms)
   50%      0.8 ms
   66%      1.0 ms
   75%      1.2 ms
   80%      1.4 ms
   90%      1.8 ms
   95%      2.5 ms
   98%      3.8 ms
   99%      5.2 ms
  100%     13.7 ms
```



and both master and volume expose metrics for prometheus  while http path /metrics   

```
➜  zergling git:(develop) ✗ curl "127.0.0.01:8080/metrics"
# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 0.25
# HELP process_max_fds Maximum number of open file descriptors.
# TYPE process_max_fds gauge
process_max_fds 1024
# HELP process_open_fds Number of open file descriptors.
# TYPE process_open_fds gauge
process_open_fds 27
....
```

