# Add --progress=plain to output build log
DOCKERFILE="-f Dockerfile"

docker build -t rapier_render $DOCKERFILE .

docker run \
  -it \
  --mount type=bind,source="$(pwd)"/data,target=/data \
  rapier_render