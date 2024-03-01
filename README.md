A small utility, written in $Blazingly$ $Fast$ $Rust$ $Programming$ $Language$🚀🚀 for sending stuff around using TCP protocol

# Usage

## Open server on receiver side

```sh
nsend receive [IP_ADDRESS]:[PORT] [FILENAME]
```

Will receive file and save it at `FILENAME``

## Send file

```sh
nsend send [IP_ADDRESS]:[PORT] [FILENAME]
```

Sends `FILENAME` to `IP_ADDRESS` via `PORT`

You can also use stdin / stdout

```sh
nsend receive [IP_ADDRESS]:[PORT] >> file.txt
```

```sh
cat file.txt | nsend send [IP_ADDRESS]:[PORT]
```

# Example

```sh

# Computer 1
nsend receive 192.168.0.5:1333 result.txt

# Computer 2
nsend send 192.168.0.5:1333 file.txt

```

