#!/bin/bash
cargo watch -w src -s "bash run_tests.sh" & pid=$!
PID_LIST+=" $pid";

cargo watch -w src -x fmt >/dev/null &
pid=$!
PID_LIST+=" $pid";

trap "kill $PID_LIST" SIGINT
wait $PID_LIST
echo "bye."
