#!/bin/sh
set -e -u

if [ $(id -u) != "0" ]
  then echo "Please run as root"
  exit
fi


# See https://llvm.org/docs/Benchmarking.html

echo "Disable address space randomization..."
echo 0 > /proc/sys/kernel/randomize_va_space

echo "Set scaling_governor to performance..."
for i in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
do
  echo "  - $i previously: `cat $i`"
  echo performance > $i
  echo "    - $i now: `cat $i`"
done
