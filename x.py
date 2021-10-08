#!/usr/bin/env python3

import functools
import os
import shutil
import subprocess
import sys


def usage():
    print(
        """
        usage: ./x.py <command> [options...] [-- <extra_options>...]
            commands: build, serve, deploy
            options:
                --dev          Create a development build. Passed to wasm-pack.
                --profiling    Create a profiling build. Passed to wasm-pack.
                --release      Create a release build. Passed to wasm-pack.
            extra options will be passed to cargo build
        """
    )


def main():
    if len(sys.argv) < 2:
        usage()
        return

    command = sys.argv[1]

    if command == "build":
        build()
    elif command == "serve":
        serve()
    elif command == "deploy":
        deploy()
    else:
        usage()
        sys.exit(1)


def run(*args):
    subprocess.run(args).check_returncode()


def build():
    run("wasm-pack", "build", "--no-typescript", "--target", "web", *sys.argv[2:])
    shutil.copyfile("pkg/caniuse_rs_bg.wasm", "public/caniuse_rs.wasm")
    run(
        "rollup", "src/main.js", "--format", "iife", "--file", "public/caniuse_rs.js",
    )
    # TODO: shutil.copytree()?
    static_files = map(lambda file: f"static/{file}", os.listdir("static"))
    run(
        "cp", "-r", *static_files, "public/",
    )


def serve():
    from http import server

    build()
    address = ('', 8000)
    handler = functools.partial(server.SimpleHTTPRequestHandler, directory="public/")
    httpd = server.HTTPServer(address, handler)
    print("Starting development server on http://localhost:8000")
    httpd.serve_forever()


def deploy():
    build()
    run("rsync", "-rzz", "public", "caniuse.rs:/tmp/caniuse/")
    run(
        "ssh",
        "caniuse.rs",
        """
        set -e
        sudo chown root: /tmp/caniuse/public
        sudo rsync -r --delete /tmp/caniuse/public/* /srv/http/caniuse.rs/
        sudo rm -r /tmp/caniuse/public
        """,
    )


if __name__ == "__main__":
    main()
