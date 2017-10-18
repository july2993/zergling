all: gen

.PHONY : gen


# $ protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` example.proto
# 

gen:
	protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` *.proto
	# protoc --rust_out=. *.proto
	# protoc --rust-grpc_out=. *.proto
