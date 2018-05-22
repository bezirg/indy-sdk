# Validation pool

The directory contains:

* The script *build-and-run-indy-cluster.sh* which creates the docker image  and starts the container with it
* The docker image configuration *indy-pool.dockerfile*
* The pool genesis file  *docker_pool_transaction_genesis* for the applications , e.g. indy-cli, to discover and to connect to the pool


The docker image is created on the base of Ubuntu 16.04 image and contains Indy components installed from pre-built packages available at sovrin.org. Note that the versions of these components are enlisted in docker configuration file. The versions may be changed in the future.

The docker container runs Indy network 'sandbox' with 4 validator nodes which use standard ports in the range 9701-9708. These ports are mapped to host ports with same corresponding numbers.

To check what has been stored in the ledger, login to the container and issue the command:

    read_ledger --type domain

To troubleshoot the setup , login to the container and issue the command:

    validator-info -v





