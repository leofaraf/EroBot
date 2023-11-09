# Инструкция по устоновке:

## Зависимости
Linux (запускал на Debian)
-  Устоновка утилиты screen для удобного переключения процессов

NodeJS 21 & npm

Nginx

Rust
- Доп. устоновка dev-утилиты `miniserve`, комманда `cargo install miniserve`, устоновка будет длиться долго
 
## Запуск
- Заходим в директорию `/src/gateway/configs/` смотрим заходим в содержимое релиз конфига NGINX  `nginx.realize.conf` и копируем его и изменяем `server_name, ssl_certificate, ssl_certificate_key` на нужный вам, после чего переходим в директорию заранее устоновленного nginx (в моем случае это был `usr/nginx/`) и отрываем nginx.conf и туда вставляем наш конфиг. Перезапускаем NGINX
- Отлично, теперь нужно загрузить SSL ключи в нужные папки (из nginx конфига ssl_certificate /etc/ssl/leofaraf.tech.crt; ssl_certificate_key /etc/ssl/leofaraf.tech.key), перезапускаем NGINX
- Открываем screen и переходим в src/webapp, там устонавливаем зависимости `npm i`, запускаем фронт `npm run start`
- Открываем новое окно screen, переходим в директорию `/src/bot/` и запускаем `miniserve -p 3002 ./`
- Открываем новое окно screen, переходим в директорию `/src/bot/` и меняем `.env` конфиг под себя, там есть довольно удобные заметки.
- В тойже папке создаем билд приложение Rust `cargo build` или `--realize` если мы окончательно хотим все соптимизировать. После чего запускаем проект, что важно в тойже папке `target/debug/userbot` или если вы указали --realize, то в  `target/realize/userbot` 
