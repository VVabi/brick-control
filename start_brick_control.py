import sys
import os
import time
import subprocess
import signal
proc = subprocess.Popen(["bluetoothctl scan on > bluetooth_log.txt&"], shell=True, preexec_fn=os.setsid)
print(proc.pid)


while True:
    os.system("bluetoothctl connect 90:84:2B:57:C6:0A > connection_attempt_log.txt")

    with open("connection_attempt_log.txt", "r") as fh:
        if "Connection successful" in fh.read():
            break
    time.sleep(1)

os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
print("CONNECTED")