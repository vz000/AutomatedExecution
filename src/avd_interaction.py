import os
import time
import sys
from threading import Thread

def start_strace(pid, file_logs):
    os.system('adb shell "timeout 10 strace -p ' + str(pid) + ' -o '+ file_logs + '"')

apkLocation = sys.argv[1] # equals sample_name
print(apkLocation)
os.system('aapt dump badging ' + apkLocation +' > aapt_output.txt')
get_line = open('aapt_output.txt','r',encoding="utf8")
try:
    first_line = get_line.readlines()[0] # processing that doesn't fit rust.
    pkg_name = first_line.split(' ')[1].split('=')[1].strip("'")
    print(pkg_name)
    os.system("adb shell monkey -p " + pkg_name + " 1")
    time.sleep(5)
    os.system('adb shell "ps -e | grep ' + pkg_name + '" > pid.txt')
    get_line = open("pid.txt",'r',encoding="utf8")
    first_line = get_line.readlines()[0] # processing that doesn't fit rust.
    process_output = first_line.split(' ')
    pid = [x for x in process_output if x != '']
    ex_strace = Thread(target=start_strace, args=(pid[1], '/data/app/logs.txt'))
    ex_strace.start()
    print("Waiting for strace to finish...")
    ex_strace.join()
except Exception as e:
    print("Deleting APK...")
    os.remove(apkLocation)
    print(e)
