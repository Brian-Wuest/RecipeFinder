# RecipeFinder

A sample client/server application for Rust and Angular

## Database

The recipe finder database has a very simple recipe storage and is perfect for small example applications.
This is currently available as a MS SQL Server database but could easily be converted over to other open-source database types.

## Server

The backing server for this application is a Rust based web server.
There are plenty of comments and examples of how to set up a rust web server with routes, authorization, response codes, and response bodies.
Rust was chosen as the server language due to it's processing speed and memory safety guarentees.

### HTTPS/TLS configuration

Make sure to download the "makecrt" executable from: <https://github.com/FiloSottile/mkcert/releases>

Run the following command to install this in the root store:

```batch
mkcert -install
```

If you want to generate your own cert/private key file, then run:

```batch
mkcert 127.0.0.1 localhost
```

This will make two .pem files in the same directory as the "mkcert" executable.
Rename the "127.0.0.1+1.pem" file to "cert" and put at the root of the /server folder.
Rename the "127.0.0.1+1-key.pem" file to "key" and put at the root of the /server folder.

## Client

The client application is an Angular front-end.
This was chosen due to familiarity with the Angular framework and community.
