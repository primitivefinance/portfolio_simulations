#!/bin/bash

forge clean

forge install transmissions11/solmate@e8f96f25d48fe702117ce76c79228ca4f20206cb --no-commit
forge install primitivefinance/portfolio@08fd1ced15373d8a871555487e9fb96d44c368ab --no-commit # g3m latest commit branch as of 9/8/23

forge bind -C lib/portfolio/contracts -b bindings/ --module  --via-ir --force --revert-strings debug
forge bind -C lib/solmate/src/tokens/WETH.sol -b bindings/ --module  --via-ir --force --revert-strings debug

echo "Completed build shell script"