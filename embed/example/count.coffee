ffi = require 'ffi'

lib = ffi.Library 'target/release/libembed.dylib', process: ['void', []]
lib.process()

console.log 'done'
