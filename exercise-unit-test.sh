#!/usr/bin/env bash

set -x

SECONDS=0

docker-compose --file exercise-stack.yaml up --detach
sleep 10
curl --silent http://10.0.0.3:8000/ping | jq --exit-status '.message == "pong"' >/dev/null || exit 1
start=$SECONDS
curl --silent http://10.0.0.3:8000/api/10 | jq --exit-status '.id == 10' >/dev/null || exit 1
exercise_duration=$((SECONDS - start))
echo "It took $exercise_duration seconds to complete the base request"
docker-compose --file exercise-stack.yaml down

if [ $exercise_duration -lt 5 ]; then
    echo "Please leave the base exercise API_WAIT_SECONDS above 5 seconds"
    exit 1
fi

docker-compose --file solution-stack.yaml up --detach
sleep 10
curl --silent http://10.0.0.3:8000/ping | jq --exit-status '.message == "pong"' >/dev/null || exit 1
start=$SECONDS
curl --silent http://10.0.0.3:8000/api/10 | jq --exit-status '.id == 10' >/dev/null || exit 1
solution_duration=$((SECONDS - start))
echo "It took $solution_duration seconds to complete the solution request"
docker-compose --file solution-stack.yaml down

if [ $solution_duration -eq 0 ]; then
    echo "The solution runs under 1 second"
    exit 0
fi

if [ $solution_duration -lt $exercise_duration ]; then
    echo "The solution runs faster than the exercise"
    echo "Consider trying to get the response under 1 second"
    exit 0
fi

echo "The solution runs slower than the exercise"
exit 1
