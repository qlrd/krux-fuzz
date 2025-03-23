import json
import os
import sys
from argparse import ArgumentParser

# add diybitcoinhardware/src/embit to PATH
# add selfcustody/src/krux to PATH
script_dir = os.path.dirname(os.path.abspath(__file__))
diybitcoinhardware_root = os.path.abspath(
    os.path.join(script_dir, "..", "..", "diybitcoinhardware", "src")
)
selfcustody_root = os.path.abspath(
    os.path.join(script_dir, "..", "..", "selfcustody", "src", "krux")
)
sys.path.insert(0, diybitcoinhardware_root)
sys.path.insert(1, selfcustody_root)

# import bbqr
from bip39 import k_mnemonic_is_valid

parser = ArgumentParser(prog="diff_bip39")
parser.add_argument("-d", "--data", type=str)


def main():
    args = parser.parse_args()
    data = json.loads(args.data)
    action = data["action"]
    input_data = data["input"]

    if action == "is_valid":
        is_valid = k_mnemonic_is_valid(input_data)
        print(json.dumps({"result": is_valid}))
    else:
        raise ValueError("Invalid action")


if __name__ == "__main__":
    main()
