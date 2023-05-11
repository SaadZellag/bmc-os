#!/usr/bin/sh
set -e

trap killgroup SIGINT

killgroup(){
  echo killing...
  kill 0
}

cargo test $1 & # In case of release mode added
sleep 1
vncviewer :5900 &
wait