eval $(tergent)
trap -- "$(
    get_trap() { echo "$3"; }
    eval "get_trap $(trap -p EXIT)"
    echo 'kill $SSH_AGENT_PID'
)" EXIT
