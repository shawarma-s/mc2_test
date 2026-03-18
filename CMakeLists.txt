cmake_minimum_required(VERSION 3.14)
project(mc2_cpp)

set(CMAKE_CXX_STANDARD 17)

# Fetch Raylib
include(FetchContent)
FetchContent_Declare(
    raylib
    URL https://github.com/raysan5/raylib/archive/refs/tags/5.0.tar.gz
)
FetchContent_MakeAvailable(raylib)

# Fetch Flecs
FetchContent_Declare(
    flecs
    URL https://github.com/SanderMertens/flecs/archive/refs/tags/v3.2.4.tar.gz
)
FetchContent_MakeAvailable(flecs)

include_directories(include)

file(GLOB SOURCES "src/*.cpp")

add_executable(mc2_cpp ${SOURCES})

target_link_libraries(mc2_cpp PRIVATE raylib flecs)

if (APPLE)
    target_link_libraries(mc2_cpp PRIVATE "-framework IOKit" "-framework Cocoa" "-framework OpenGL")
endif()
