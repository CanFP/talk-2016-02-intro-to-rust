#!/usr/bin/env python

from ctypes import cdll

def main():
    lib = cdll.LoadLibrary("embed/target/release/libembed.dylib")
    lib.process()
    print("done!")

if __name__=="__main__":
    main()
