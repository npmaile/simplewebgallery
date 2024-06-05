# SimpleWebGallery
This is a simple media browser.

## Building
`cargo build`

## Usage:
`DATA_DIR=/path/to/the/directory/you/wish/to/serve target/release/simplewebgallery`

the docker compose one uses nginx to host the static content which works about one zillion times better than using the actix web static host.
## Better Usage
`ROOT_DIR=/path/to/the/directory/you/wish/to/serve docker compose up`
