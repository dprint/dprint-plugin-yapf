import sys
import os
packages_dir = os.path.normpath(
    os.path.dirname(os.path.realpath(__file__)) + "/packages")
sys.path.insert(0, str(packages_dir))

import argparse
import struct
import json
import io
from yapf.yapflib.yapf_api import FormatCode
import traceback

BUFFER_SIZE = 1024


def eprint(message):
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


def send_success(stdout):
    write_int(stdout, 0)  # success
    stdout.flush()


def send_failure(stdin, stdout, message):
    encoded_message = message.encode('utf-8')
    write_int(stdout, 1)  # error
    write_variable_data(stdin, stdout, encoded_message)


def send_int(stdout, value):
    write_int(stdout, 0)  # success
    write_int(stdout, value)
    stdout.flush()


def write_int(stdout, value):
    stdout.write(struct.pack('>I', value))


def read_int(stdin):
    value = bytearray(4)
    bytes_read = stdin.readinto(value)
    return struct.unpack_from('>I', value)[0]


def send_string(stdin, stdout, value):
    encoded_value = value.encode('utf-8')
    write_int(stdout, 0)  # success
    write_variable_data(stdin, stdout, encoded_value)


def write_variable_data(stdin, stdout, value):
    size = len(value)
    write_int(stdout, size)
    strict_write(stdout, value[0:min(size, BUFFER_SIZE)])
    stdout.flush()

    index = BUFFER_SIZE
    while index < size:
        # wait for "ready" from the client
        read_int(stdin)

        # write to buffer
        end_index = min(index + BUFFER_SIZE, size)
        bytes_written = strict_write(stdout, value[index:end_index])
        stdout.flush()

        index = end_index


def strict_write(stdout, view):
    bytes_written_len = stdout.write(view)
    if bytes_written_len != len(view):
        eprint("Bytes written of " + bytes_written_len +
               " did not equal the expected length of " + len(view))
        process.exit(1)


def read_string(stdin, stdout):
    data = read_variable_data(stdin, stdout)
    return data.decode('utf-8')


def read_variable_data(stdin, stdout):
    size = read_int(stdin)
    result = bytearray(size)
    message_data = memoryview(result)
    if size > 0:
        index = 0
        end_index = min(size, BUFFER_SIZE)
        strict_read(stdin, message_data[index:end_index])

        index = end_index

        while index < size:
            # send "ready" to client
            write_int(stdout, 0)
            stdout.flush()

            end_index = min(index + BUFFER_SIZE, size)
            strict_read(stdin, message_data[index:end_index])

            index = end_index

    return result


def strict_read(stdin, view):
    bytes_read_len = stdin.readinto(view)
    if bytes_read_len != len(view):
        eprint("Bytes read of " + bytes_read_len +
               " did not equal the expected length of " + len(view))
        process.exit(1)


def get_license_text():
    return """yapf: Apache License 2.0

The MIT License (MIT)

Copyright (c) 2020 David Sherret

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."""


def get_dprint_to_yapf_config(config):
    final_config = {}
    for key in config:
        if key == "lineWidth":
            if config[key] is not None:
                final_config["column_limit"] = config[key]
        elif key == "indentWidth":
            if config[key] is not None:
                final_config["indent_width"] = config[key]
        elif key == "useTabs":
            if config[key] is not None:
                final_config["use_tabs"] = config[key]
        elif key == "newLineKind":
            # doesn't seem to exist in yapf
            pass
        else:
            final_config[key.lower()] = config[key]
    return final_config


def get_resolved_config(global_config, plugin_config, additional_config):
    resolved_config = {}
    resolved_config.update(get_dprint_to_yapf_config(global_config))
    resolved_config.update(get_dprint_to_yapf_config(plugin_config))
    resolved_config.update(get_dprint_to_yapf_config(additional_config))
    return resolved_config


def get_style_config_from_config(config):
    text = "{"
    i = 0
    for key in config:
        if i > 0:
            text += ", "
        text += key + ": " + str(config[key])
        i += 1
    text += "}"
    return text


parser = argparse.ArgumentParser()
parser.add_argument('--parent-pid',
                    type=int,
                    help='The parent process\' process identifier.')
args = parser.parse_args()

# disable buffering
with io.open(sys.stdin.fileno(), 'rb', buffering=0) as stdin:
    with io.open(sys.stdout.fileno(), 'wb', buffering=0) as stdout:
        global_config = {}
        plugin_config = {}

        while True:
            message_kind = read_int(stdin)

            try:
                if message_kind == 8:  # close
                    sys.exit(0)
                    break
                elif message_kind == 0:  # get plugin schema version
                    send_int(stdout, 2)
                elif message_kind == 1:  # get plugin info
                    send_string(
                        stdin, stdout,
                        json.dumps({
                            "name": "dprint-plugin-yapf",
                            "version": "0.1.0",
                            "configKey": "yapf",
                            "fileExtensions": ["py"],
                            "helpUrl": "https://dprint.dev/plugins/yapf",
                            "configSchemaUrl": ""
                        }))
                elif message_kind == 2:  # get license text
                    send_string(stdin, stdout, get_license_text())  #todo
                elif message_kind == 3:  # get resolved config
                    send_string(stdin, stdout, "{}")  # todo
                elif message_kind == 4:  # set global config
                    global_config = json.loads(read_string(stdin, stdout))
                    send_success(stdout)
                elif message_kind == 5:  # set plugin config
                    plugin_config = json.loads(read_string(stdin, stdout))
                    send_success(stdout)
                elif message_kind == 6:  # get configuration diagnostics
                    send_string(stdin, stdout, "[]")  # todo
                elif message_kind == 7:  # format text
                    file_path = read_string(stdin, stdout)  # todo: file path
                    file_text = read_string(stdin, stdout)
                    override_config = json.loads(read_string(stdin, stdout))

                    resolved_config = get_resolved_config(
                        global_config, plugin_config, override_config)
                    style_config = get_style_config_from_config(
                        resolved_config)

                    (formatted_text, _) = FormatCode(file_text,
                                                     style_config=style_config)
                    if file_text == formatted_text:
                        send_int(stdout, 0)  # no change
                    else:
                        encoded_text = formatted_text.encode('utf-8')
                        write_int(stdout, 0)  # success
                        write_int(stdout, 1)  # file changed
                        write_variable_data(stdin, stdout, encoded_text)
                else:
                    raise ValueError("Unexpected message kind: " +
                                     str(message_kind))
            except Exception as error:
                send_failure(stdin, stdout, traceback.format_exc())
