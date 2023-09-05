#!/bin/bash

forge clean

forge install transmissions11/solmate@e8f96f25d48fe702117ce76c79228ca4f20206cb --no-commit
forge install primitivefinance/portfolio@88e6fc8e24e182216a7788df8871f1dc2553755a --no-commit # latest changes here for now.

forge bind -C lib/portfolio/contracts -b bindings/ --module --overwrite --via-ir --force --revert-strings debug
forge bind -C lib/solmate/src/tokens/WETH.sol -b bindings/ --module --overwrite --via-ir --force --revert-strings debug

echo "Completed build shell script"