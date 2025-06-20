cmake_minimum_required(VERSION 3.10)
project(AI-Playground VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Set compiler flags
if(MSVC)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W4")
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic")
endif()

# Set output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Find packages (for future HTTP client libraries)
# find_package(CURL REQUIRED)
# find_package(nlohmann_json REQUIRED)

# Collect source files
set(SOURCES
    src/main.cpp
    src/components/AIComponent.cpp
    src/components/LLMInterface.cpp
    src/components/PromptManager.cpp
)

# Add executable
add_executable(AI-Playground ${SOURCES})

# Set properties for the executable
set_target_properties(AI-Playground PROPERTIES
    OUTPUT_NAME "ai-playground"
    DEBUG_POSTFIX "_d"
)

# Link libraries (uncomment when adding HTTP client)
# target_link_libraries(AI-Playground PRIVATE CURL::libcurl nlohmann_json::nlohmann_json)

# Include directories
target_include_directories(AI-Playground PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Install rules
install(TARGETS AI-Playground
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)

# Print configuration info
message(STATUS "AI-Playground Configuration:")
message(STATUS "  Version: ${PROJECT_VERSION}")
message(STATUS "  C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "  Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "  Output Directory: ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}")

