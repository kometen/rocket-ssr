Write a message and encrypt it on your device using javascript and store the encrypted message on the server.
Share the encrypted message using a link and provide the recipient with the key downloaded to your device.

I had much help from Claude AI.

Decrypting the message can be done without registering an account. Creating encrypted messages requires a registration.
You can choose whatever username and alias you want. The login-information is registered on your device as a passkey.
The alias is hashed at Bitwarden and cannot be shown in plaintext, the username is only known to you.

The message can be decrypted on your device using the passkey.

To see it in action, visit the following link:

https://obscura.bsky.dk/message/gKSxqMBjzj1Su0QAViN6zg

and copy and paste the decryption key `sSjMGidFJzDuG6h2LNsxBHGr/dwgIoP+CWAOGgxRXZ8=` into the input field.

---

To run this on your local machine, clone this repository, create an account at `https://admin.passwordless.dev/signup`, create
an application after registering. A public and private key pair is generated for you. Save these keys in a safe place, for instance
in a password manager.

```
git clone git@github.com:kometen/rocket-ssr.git
cd rocket-ssr
cat << 'EOF' > ./.env
PASSWORDLESS_API_KEY=myapp:public:foobarbaz
PASSWORDLESS_API_SECRET=myapp:secret:foobarbaz
PASSWORDLESS_API_URL=https://v4.passwordless.dev
EOF
cargo run
```

Then open your browser at `http://localhost:8008`.

Run in a container.

```
cat << 'EOF' > ./.env
PASSWORDLESS_API_KEY=myapp:public:foobarbaz
PASSWORDLESS_API_SECRET=myapp:secret:foobarbaz
PASSWORDLESS_API_URL=https://v4.passwordless.dev
EOF
docker build -t rocket-ssr:dev .
docker run -e ROCKET_ADDRESS=0.0.0.0 -p 8008:8008 rocket-ssr:dev
```

---

Register account (optional).

![Register account](./doc/register.png)

Store passkey (optional).

![Store passkey](./doc/passkey.png)

Write message (optional).

![Write message](./doc/write.png)

Download key (optional).

![Download key](./doc/download.png)

Decrypt message using provided key.

![Decrypt message](./doc/decrypt.png)

Login based on an example at https://github.com/davidzr/passwordless-rust and the Bitwarden passwordless
service at https://bitwarden.com/products/passwordless/.
