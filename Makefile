
get-map:
	mkdir -p ./valhalla/files
	wget -O ./valhalla/files/michigan-latest.osm.pbf \
	 		https://download.geofabrik.de/north-america/us/michigan-latest.osm.pbf

docker-run:
	docker run -dt --rm --name valhalla \
	-p 8002:8002 \
	-v ./valhalla/files:/custom_files \
	-e tile_urls=http://download.geofabrik.de/north-america/us/michigan-latest.osm.pbf \
	-e serve_tiles=True -e build_admins=True \
	ghcr.io/nilsnolde/docker-valhalla/valhalla:latest

podman-run:
	podman run -dt --rm --name valhalla \
	-p 8002:8002 \
	-v ./valhalla/files:/custom_files \
	-e tile_urls=http://download.geofabrik.de/north-america/us/michigan-latest.osm.pbf \
	-e serve_tiles=True -e build_admins=True \
	ghcr.io/nilsnolde/docker-valhalla/valhalla:latest
