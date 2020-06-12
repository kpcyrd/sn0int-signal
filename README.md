# sn0int-signal

Provides a simple api for the kpcyrd/notify-signal sn0int module.

You need a second phone number to register with signal. A landline number
could work for this.

## Setup

Make sure you have [`signal-cli`](https://github.com/AsamK/signal-cli)
installed, setup, and in your path!

Afterwards install the http api like this:

    git clone 'https://github.com/kpcyrd/sn0int-signal'
    cd sn0int-signal
    cargo install --path .

## Usage

    sudo install -m 600 -o "$USER" /dev/null /etc/sn0int-signal.key
    < /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1 > /etc/sn0int-signal.key
    sn0int-signal -k /etc/sn0int-signal.key 127.0.0.1:4321 +31337

## Testing

    curl -v -H 'Content-Type: application/json' \
        -H "x-signal-auth: $(cat /etc/sn0int-signal.key)" \
        -d '{"to": "+313372", "body": "ohai"}' \
        http://127.0.0.1:4321/api/v0/send

## Start on boot

    # /etc/systemd/system/sn0int-your-other-service.service

    [Unit]
    Description=sn0int-signal: api for signal notifications

    [Service]
    User=your-user
    ExecStart=/usr/local/bin/sn0int-signal -k /etc/sn0int-signal.key 127.0.0.1:4321 +31337

## License

GPLv3+
