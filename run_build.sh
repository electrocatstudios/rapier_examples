# Add --progress=plain to output build log
DOCKERFILE="-f Dockerfile"
BUILD_ARGS="$1"

docker build -t rapier_render $DOCKERFILE . --build-arg build_options="$BUILD_ARGS"

docker run \
  -it \
  --mount type=bind,source="$(pwd)"/data,target=/data \
  rapier_render