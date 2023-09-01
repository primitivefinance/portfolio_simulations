#!/bin/bash

forge clean

forge install transmissions11/solmate@e8f96f25d48fe702117ce76c79228ca4f20206cb --no-commit
forge install primitivefinance/portfolio@78fc43dfa22d45f2af0d39e9302bd8701c41652d --no-commit

#forge bind -C lib/portfolio/contracts -b src/bindings/ --module --overwrite --via-ir --force --revert-strings debug
#forge bind -C lib/solmate/src/tokens/WETH.sol -b src/bindings/ --module --overwrite --via-ir --force --revert-strings debug

forge bind -b src/bindings/ --module --overwrite --via-ir --force --revert-strings debug

echo "Completed build shell script"