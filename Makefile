
get-map:
	mkdir -p ./valhalla/files
	wget -O ./valhalla/files/michigan-latest.osm.pbf \
	 		https://download.geofabrik.de/north-america/us/michigan-latest.osm.pbf

docker-run:
	docker run -dt --rm --name valhalla \
           	-p 8002:8002 \
           	-v ./valhalla/files:/custom_files \
           	-e tile_urls=http://download.geofabrik.de/north-america/us/michigan-latest.osm.pbf \
           	-e serve_tiles=True \
           	ghcr.io/nilsnolde/docker-valhalla/valhalla:3.5.1

podman-run:
	podman run -dt --rm --name valhalla \
           	-p 8002:8002 \
           	-v ./valhalla/files:/custom_files \
           	-e tile_urls=http://download.geofabrik.de/north-america/us/michigan-latest.osm.pbf \
           	-e serve_tiles=True \
           	ghcr.io/valhalla/valhalla-scripted:latest
           	# ghcr.io/nilsnolde/docker-valhalla/valhalla:3.5.1

build:
	cargo run --package evedb --bin evedb -- \
	--repo-path ~/data/eved/repo \
	--db-path ~/data/eved/db/eved.db \
	--verbose build --no-clean --no-clone

match:
	cargo run --package evedb --bin evedb -- \
	--db-path ~/data/eved/db/eved.db \
	--verbose match

match-r:
	cargo run --release --package evedb --bin evedb -- \
	--db-path ~/data/eved/db/eved.db \
	--verbose match

build-r:
	cargo run --release --package evedb --bin evedb -- \
	--repo-path ~/data/eved/repo \
	--db-path ~/data/eved/db/eved.db \
	--verbose build --no-clean --no-clone

flamegraph:
	cargo flamegraph --package evedb --bin evedb -- \
	--repo-path ~/data/eved/repo \
	--db-path ~/data/eved/db/eved.db \
	--verbose build --no-clean --no-clone

samply:
	samply record cargo run --release --package evedb --bin evedb -- \
	  --repo-path ~/data/eved/repo \
	  --db-path ~/data/eved/db/eved.db \
	  --verbose build --no-clean --no-clone

prune-docker:
	docker system prune --all --force --volumes

prune-podman:
	podman system prune --all --force --volumes
