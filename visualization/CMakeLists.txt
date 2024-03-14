cmake_minimum_required(VERSION 3.5)

project(bfd_visualization VERSION 0.1 LANGUAGES C CXX)

option(BUILD_UT "Option to enable building of unit tests." OFF)
option(BUILD_GRPC "Option to fetch and build grpc lib. Required on the first build." OFF)
option(GRPC_MIDDLEWARE "Option to use gRPC as a communication. Only one commmunication type should be enable." ON)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20 -Wall -Werror -ggdb -Wextra -Wno-unused-parameter -fPIC -pthread")

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets)

add_subdirectory(src)
if (BUILD_UT)
    add_subdirectory(tests)
endif()