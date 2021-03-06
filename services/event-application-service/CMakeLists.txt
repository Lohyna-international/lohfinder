cmake_minimum_required(VERSION 3.21)

set(CMAKE_CXX_COMPILER "clang++")
set(CMAKE_C_COMPILER "clang")
set(CMAKE_LINKER "/usr/bin/lld")

set(CMAKE_TOOLCHAIN_FILE "/vcpkg/scripts/buildsystems/vcpkg.cmake")

project(afs)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(google_cloud_cpp_pubsub REQUIRED)
# find_package(bsoncxx CONFIG REQUIRED)
# find_package(mongocxx CONFIG REQUIRED)
find_package(glog CONFIG REQUIRED)
find_package(GTest CONFIG REQUIRED)

set(PUBSUB_CONTROLLER_SRCS
  src/pubsub_controller/connection_factory.cc
  src/pubsub_controller/connection_factory.h
  src/pubsub_controller/pubsub_controller.cc
  src/pubsub_controller/pubsub_controller.h
  src/pubsub_controller/pubsub_subscribers.h
  src/pubsub_controller/pubsub_thread_pool.cc
  src/pubsub_controller/pubsub_thread_pool.h
)

set(QUERIES_SRCS
  src/queries/domain.h
  src/queries/queries.h
)

set(COMMANDS_SRCS
  src/commands/domain.h
  src/commands/commands_handler.h
)

set(BUILD_FLAGS
  $<$<CONFIG:Debug>:-fsanitize=address -fno-omit-frame-pointer>
  $<$<CONFIG:Release>:-O3>
  -Wall -Wpedantic -Werror -Wextra)

add_executable(afs 
  src/main.cc
  src/application.cc
  src/application.h
  src/utils/signal_handler.h
  ${PUBSUB_CONTROLLER_SRCS}
  ${QUERIES_SRCS}
  ${COMMANDS_SRCS}
)

target_compile_features(afs PRIVATE cxx_std_17)
target_compile_options(afs PUBLIC ${BUILD_FLAGS})
target_link_options(afs PUBLIC  $<$<CONFIG:Debug>:-fsanitize=address>)
target_link_libraries(afs google-cloud-cpp::pubsub glog::glog) # mongo::bsoncxx_static mongo::mongocxx_static)
target_include_directories(afs PUBLIC src)

add_executable(afs_test
  tests/mocks/connection_factory_mock.h
  tests/mocks/commands_handler_mock.h
  tests/mocks/query_handler_mock.h
  tests/pubsub_controller_test.cc
  ${PUBSUB_CONTROLLER_SRCS}
  ${QUERIES_SRCS}
  ${COMMANDS_SRCS})
target_compile_features(afs_test PRIVATE cxx_std_17)
target_compile_options(afs_test PUBLIC -Wall -Wpedantic -Werror -Wextra)
target_include_directories(afs_test PUBLIC src tests)
target_link_libraries(afs_test PRIVATE GTest::gmock 
                                       GTest::gtest 
                                       GTest::gmock_main 
                                       GTest::gtest_main 
                                       google-cloud-cpp::pubsub 
                                       glog::glog)
