### Quick start

The front-end web app is built on top of [Vuestic Admin](https://admin.vuestic.dev) with [Vuestic UI](https://ui.vuestic.dev).

### Development

To host webr static files with nginx, mainly refer to https://docs.r-wasm.org/webr/latest/serving.html:

1. Download the latest binaries from https://github.com/r-wasm/webr/releases, unzip and move to `/var/www`

2. Serve the binaries with nginx

   ```
   location /webr {
     add_header Access-Control-Allow-Origin "*";
     add_header Access-Control-Expose-Headers "*";
     alias /var/www/webr-0.5.8/;
   }
   ```

To host the filesystem containin the preload packages, https://docs.r-wasm.org/webr/latest/mounting.html, https://r-wasm.github.io/rwasm/articles/mount-fs-image.html

1. Build the filesystem

   ```
   docker run -it --rm -v ${PWD}/output:/output -w /output ghcr.io/r-wasm/webr:main R

   # run builder.R
   ```

2. To host the vfs files

   ```
   location /vfs {
     location ~* \.(gz)$ {
       add_header Content-Encoding gzip;
       add_header Access-Control-Allow-Origin "*";
     }
     add_header Access-Control-Allow-Origin "*";
     alias /var/www/vfs/;
   }
   ```
