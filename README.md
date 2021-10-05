# hellfire-backend
The headless CMS

For normal development, you will need local instance of mongodb and rust-nightly installed.

For mongodb, set the env variab;e `MONGO_URL` to the correct url. If not set, it will default to `localhost:27017`. You can also set an optional `PORT` env variable

Then run `cargo run` in the terminal in the directory of this repo to start the server.

To interact with the server, you will probably need the admin panel too [link](https://github.com/HellFireCMS/hellfire-admin)
