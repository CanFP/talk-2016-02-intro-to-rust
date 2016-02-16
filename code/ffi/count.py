#!/usr/bin/env python

import threading

class CounterThread(threading.Thread):
    def __init__(self, n):
        threading.Thread.__init__(self)
        self.n = n
    def run(self):
        x = 0
        for i in range(self.n):
            x = x+1

def main():
    threads = []
    for i in range(10):
        t = CounterThread(5000000)
        t.start()
        threads.append(t)
    for t in threads:
        print("Thread finished with count=%d" % t.n)
        t.join()
    print("done!")

if __name__=="__main__":
    main()
