#!/bin/bash

forge clean

forge install transmissions11/solmate@e8f96f25d48fe702117ce76c79228ca4f20206cb --no-commit
forge install primitivefinance/portfolio@728b04f29c1e66875d5fdac7e24cd0422ad17caa --no-commit

forge bind -C lib/portfolio/contracts -b src/bindings/ --module --overwrite --via-ir --force
forge bind -C lib/solmate/src/tokens/WETH.sol -b src/bindings/ --module --overwrite --via-ir --force

echo "Completed build shell script"