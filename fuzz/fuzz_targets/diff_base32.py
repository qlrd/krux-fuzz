import json
import os
import sys
from argparse import ArgumentParser

script_dir = os.path.dirname(os.path.abspath(__file__))
bbqr_path = os.path.abspath(os.path.join(script_dir, "..", "..", "BBQr", "python"))
sys.path.insert(0, bbqr_path)
krux_path = os.path.abspath(os.path.join(script_dir, "..", "..", "selfcustody", "src"))
sys.path.insert(0, krux_path)


from krux.bbqr import base32_decode_stream, base32_encode_stream

from bbqr.utils import decode_data, encode_data


def main():
    parser = ArgumentParser(prog="diff_base32")
    parser.add_argument("-d", "--data", type=str)
    args = parser.parse_args()

    data = json.loads(args.data)
    module = data["module"]
    action = data["action"]
    padding = data["padding"]
    input_data = bytes(data["input"])

    if module == "krux":
        if action == "encode":
            encoded = "".join(base32_encode_stream(input_data, add_padding=padding))
            print(json.dumps({"result": encoded}))
        elif action == "decode":
            decoded = base32_decode_stream(input_data.decode("utf-8"))
            print(json.dumps({"result": list(decoded)}))
        else:
            raise ValueError(f"Invalid module '{module}' for module krux")

    elif module == "bbqr":
        if action == "encode":
            encoded = encode_data(input_data)
            print(json.dumps({"result": encoded[1]}))
        else:
            raise ValueError(f"Invalid action '{action}' for module 'bbqr'")

    else:
        raise ValueError(f"Invalid module '{module}'")


if __name__ == "__main__":
    main()
