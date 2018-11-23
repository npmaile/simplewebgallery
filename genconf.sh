#!/usr/bin/bash
echo -e "
events{
	worker_connections 4096;
}
http {
	server{
		location / {
			root "$(pwd)"/data;
			index index.php;
			disable_symlinks off;
			}
		location ~ [^/]\\.php(/|$) {
			fastcgi_split_path_info ^(.+?\\.php)(/.*)$;
			if (!-f \$document_root\$fastcgi_script_name){
				return 404;
				}
				root "$(pwd)"/data;	
			fastcgi_param HTTP_PROXY \"\";
			fastcgi_pass unix:/var/run/php-fpm/php-fpm.sock;
			fastcgi_index index.php;
			include /etc/nginx/fastcgi_params;
			}
	}
}"
