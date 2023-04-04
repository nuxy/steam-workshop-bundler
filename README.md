# steam-workshop-bundler

Manage multiple [Steam workshops](https://steamcommunity.com/workshop) for a _single game instance_.

## Dependencies

- [Visual Studio Code](https://code.visualstudio.com/download)
- [Docker](https://docs.docker.com/get-docker)

## Usage

    $ ./steam-workshop-bundler

    USAGE:
        steam-workshop-bundler [OPTIONS] --username <USERNAME> --password <PASSWORD> --guard-code <GUARD_CODE> --workshop <WORKSHOP>

    OPTIONS:
            --username <USERNAME>        Steam account Username.
            --password <PASSWORD>        Steam account password.
            --guard-code <GUARD_CODE>    Steam Guard code.
            --workshop <WORKSHOP>        Workshop name to publish.
            --generate                   Generate workshop sources (optional).
            --public                     Adds workshop to Steam results (optional).
        -h, --help                       Print help information
        -V, --version                    Print version information

## Launching in Remote-Containers

In the VS Code _Command Palette_ choose "Open Folder in Container" which will launch the server in a Docker container allowing for realtime development and testing.

## Developers

### CLI options

Run [Cargo](https://doc.rust-lang.org/stable/cargo/commands) on project sources:

    $ cargo build
    $ cargo fmt

Debug _*binary_ output:

    $ ./target/debug/steam-workshop-bundler

(*) Due to `steamcmd.exe` running in a Windows context, while the underlying Dev Container OS is Linux, the generated VDF will contain invalid paths (e.g. Linux paths).  This will result with a failure during the Steam workshop publish process.  That being said, final testing occur using the target OS.

## References

- [Steam Workshop Implementation Guide](https://partner.steamgames.com/doc/features/workshop/implementation)

## Contributions

If you fix a bug, or have a code you want to contribute, please send a pull-request with your changes.

## Versioning

This package is maintained under the [Semantic Versioning](https://semver.org) guidelines.

## License and Warranty

This package is distributed in the hope that it will be useful, but without any warranty; without even the implied warranty of merchantability or fitness for a particular purpose.

_steam-workshop-bundler_ is provided under the terms of the [MIT license](http://www.opensource.org/licenses/mit-license.php)

[Steam](https://store.steampowered.com) is a registered trademark of Valve Corporation.

## Author

[Marc S. Brooks](https://github.com/nuxy)
