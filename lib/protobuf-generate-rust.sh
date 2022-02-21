#!/bin/bash
# protoc --plugin=`which protoc-gen-rust`   -Iproto proto/* --rust_out=rsout
protoc --plugin=`which protoc-gen-rust`   -Iproto proto/* --rust_out=src/proto
