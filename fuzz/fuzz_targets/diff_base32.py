import os
import sys
import json
from argparse import ArgumentParser

script_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.abspath(os.path.join(script_dir, '..', '..'))
sys.path.insert(0, project_root)

from krux.src.bbqr import base32_encode_stream, base32_decode_stream

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
