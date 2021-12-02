FROM rlevkovych/lohfindercpp:latest

COPY . /app
# Building an executable.
RUN mkdir -p /app/build
RUN cmake -S /app -B /app/build -G Ninja -D CMAKE_TOOLCHAIN_FILE=/deps/vcpkg/scripts/buildsystems/vcpkg.cmake -D CMAKE_BUILD_TYPE="Release"
RUN cmake --build /app/build

CMD ./app/build/afs $APP_NAME
