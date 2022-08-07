# ELA-API (Easy Logging Application)

## Description
API backend for the ELA project. Records all pings in a database and displays them visually on a website.

## How to setup
- copy `docker-compose.yaml` file
- run `docker-compose up` in the directory with the `docker-compose.yaml` file

## Important paths
- `/var/lib/ela/db.sqlite`
- - The database

## Routes
- `/`
- - Website (for displaying data)
- `/api/data`
- - online data as json
- `/api/ping/<uuid>`
- -to ping the api