import json
import os
import sys
from argparse import ArgumentParser

# add selfcustody/src/krux to PATH
script_dir = os.path.dirname(os.path.abspath(__file__))
selfcustody_root = os.path.join(script_dir, "..", "..", "selfcustody")
project_root = os.path.join(selfcustody_root, "src", "krux")
sys.path.insert(0, os.path.abspath(project_root))

# import bbqr
from bbqr import base32_decode_stream, base32_encode_stream

parser = ArgumentParser(prog="diff_base32")
parser.add_argument("-d", "--data", type=str)


def main():
    args = parser.parse_args()

    data = json.loads(args.data)
    action = data["action"]
    padding = data["padding"]
    input_data = bytes(data["input"])

    if action == "encode":
        encoded = "".join(base32_encode_stream(input_data, add_padding=padding))
        print(json.dumps({"result": encoded}))
    elif action == "decode":
        decoded = base32_decode_stream(input_data.decode("utf-8"))
        print(json.dumps({"result": list(decoded)}))
    else:
        raise ValueError("Invalid action")


if __name__ == "__main__":
    main()
