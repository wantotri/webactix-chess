# Webactix-Chess

Multi-player Chess Game created using Rust Actix Web

## How to Deploy

1. Change the tera template folder in `main.rs` for `Tera` to
    ```rust
    let tera = Tera::new("/home/ubuntu/webactix/templates/**/*").unwrap();
    ```

2. Compiled the App
    ```bash
    cargo build --release
    ```

3. Copy the compiled file and the template dir to server
    ```bash
    # Note: create the dir first if not exists
    scp target/release/webactix wantotrees:~/webactix/target/release/
    scp -r templates/* wantotrees:~/webactix/templates/
    ```

4. Connect to Server then create service file, save it in `/etc/systemd/system/webactix-chess.service`
    ```
    [Unit]
    Description=Webactix App for Chess App
    After=network.target

    [Service]
    User=ubuntu
    Group=www-data
    WorkingDirectory=/home/ubuntu/webactix
    ExecStart=/home/ubuntu/webactix/target/release/webactix

    [Install]
    WantedBy=multi-user.target
    ```

5. Update the Nginx config file in `/etc/nginx/sites-available/default/`
    ```nginx
    server {
        listen 80;
        server_name chess.wantotrees.xyz;
        root /home/ubuntu/webactix;

        location / {
            proxy_pass http://localhost:7878;
            proxy_redirect off;
            proxy_set_header Host $host;
            proxy_set_header X-Real-Ip $remote_addr;
        }

        location /static {
            alias /home/ubuntu/webactix/static/;
        }

        location /ws/ {
            proxy_pass http://localhost:7878/ws/;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_read_timeout 120s;
            proxy_send_timeout 120s;
        }
    }
    ```

6. Restart the Nginx service
    ```bash
    # Check the nginx syntax
    sudo nginx -t

    # Restart the service
    sudo systemctl restart nginx
    ```

7. Start and Enable `webactix-chess.service`
    ```bash
    # Start the service
    sudo systemctl start webactix-chess

    # Enable the service at startup
    sudo systemctl enable webactix-chess
    ```


## Note

to see the service log

```
journalctl -u webactix-chess.service
```