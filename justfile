fmt:
    make fmt

lint:
    make lint

test:
    make test

ci:
    just fmt && just lint && just test

doc:
    make doc