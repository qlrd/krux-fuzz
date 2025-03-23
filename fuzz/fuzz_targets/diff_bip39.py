import json
import os
import sys
from argparse import ArgumentParser

# add diybitcoinhardware/src/embit to PATH
# add selfcustody/src/krux to PATH
script_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.abspath(os.path.join(script_dir, "..", ".."))
diybitcoinhardware_root = os.path.abspath(
    os.path.join(project_root, "diybitcoinhardware", "src")
)
krux_root = os.path.abspath(os.path.join(project_root, "selfcustody", "src"))
sys.path.insert(0, diybitcoinhardware_root)
sys.path.insert(1, krux_root)


# import both bip39 modules of embit and krux
from embit import bip39 as embit_bip39
from krux import bip39 as krux_bip39

parser = ArgumentParser(prog="diff_bip39")
parser.add_argument("-d", "--data", type=str)


def main():
    args = parser.parse_args()
    data = json.loads(args.data)
    action = data["action"]
    module = data["module"]

    if module == "embit":
        input_data = bytearray(data["input"])
        method = getattr(embit_bip39, action)
        result = method(input_data)

    elif module == "krux" and action == "k_mnemonic_bytes":
        input_data = data["input"]
        method = getattr(krux_bip39, action)
        result = list(method(input_data))

    else:
        raise ValueError("Invalid action")

    print(json.dumps({"result": result}))


if __name__ == "__main__":
    main()
