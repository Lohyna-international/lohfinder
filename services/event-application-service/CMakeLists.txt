cmake_minimum_required(VERSION 3.21)

set(CMAKE_CXX_COMPILER "clang++")
set(CMAKE_C_COMPILER "clang")

set(CMAKE_TOOLCHAIN_FILE "/deps/vcpkg/scripts/buildsystems/vcpkg.cmake")

project(afs)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(google_cloud_cpp_pubsub REQUIRED)
find_package(bsoncxx CONFIG REQUIRED)
find_package(mongocxx CONFIG REQUIRED)
find_package(glog CONFIG REQUIRED)

add_executable(afs 
  src/main.cc
  src/application.h
  src/application.cc
  src/pubsub_communication/pubsub_controller.h
  src/pubsub_communication/pubsub_controller.cc
  src/pubsub_communication/pubsub_subscribers.h
  src/pubsub_communication/pubsub_subscribers.cc
  src/utils/signal_handler.h
)

set(BUILD_FLAGS
  $<$<CONFIG:Debug>:-fsanitize=address -fno-omit-frame-pointer>
  $<$<CONFIG:Release>:-O3>
  -Wall -Wpedantic -Werror -Wextra)

target_compile_features(afs PRIVATE cxx_std_17)
target_compile_options(afs PUBLIC ${BUILD_FLAGS})
target_link_options(afs PUBLIC  $<$<CONFIG:Debug>:-fsanitize=address>)
target_link_libraries(afs google-cloud-cpp::pubsub glog::glog) # mongo::bsoncxx_static mongo::mongocxx_static)
target_include_directories(afs PUBLIC src)