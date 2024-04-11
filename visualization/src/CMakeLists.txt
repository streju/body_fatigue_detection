set(PROTOBUF_INC_DIR "${PROJECT_SOURCE_DIR}/src/proto_include")
file(MAKE_DIRECTORY ${PROTOBUF_INC_DIR})
set(LINKED_LIBS)

include(middleware/middleware.cmake)

set(PROJECT_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/mainwindow.cpp
    
    ${CMAKE_CURRENT_SOURCE_DIR}/mainwindow.ui
    ${CMAKE_CURRENT_SOURCE_DIR}/middleware/grpc/ImageListener.cpp
    ${PROTOBUF_INC_DIR}/alerts.grpc.pb.cc
    ${PROTOBUF_INC_DIR}/alerts.pb.cc
    ${PROTOBUF_INC_DIR}/common.grpc.pb.cc
    ${PROTOBUF_INC_DIR}/common.pb.cc
    ${PROTOBUF_INC_DIR}/image.grpc.pb.cc
    ${PROTOBUF_INC_DIR}/image.pb.cc
    ${PROTOBUF_INC_DIR}/body_info.grpc.pb.cc
    ${PROTOBUF_INC_DIR}/body_info.pb.cc
    ${PROTOBUF_INC_DIR}/visualization_service.grpc.pb.cc
    ${PROTOBUF_INC_DIR}/visualization_service.pb.cc
)
set(INCLUDES_DIR
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/middleware
    ${CMAKE_CURRENT_SOURCE_DIR}/middleware/grpc
    ${CMAKE_CURRENT_SOURCE_DIR}/utils
    ${PROTOBUF_INC_DIR}
    ${Protobuf_INCLUDE_DIRS}
)

if(${QT_VERSION_MAJOR} GREATER_EQUAL 6)
    qt_add_executable(${PROJECT_NAME}
        MANUAL_FINALIZATION
        ${PROJECT_SOURCES}
    )
else()
    add_executable(visualization
        ${PROJECT_SOURCES}
    )
endif()

########## OPENCV ##########
find_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})
list(APPEND LINKED_LIBS ${LINKED_LIBS} ${OpenCV_LIBS})

########## NLOHMANN ##########
include(FetchContent)
FetchContent_Declare(json URL https://github.com/nlohmann/json/releases/download/v3.11.3/json.tar.xz)
FetchContent_MakeAvailable(json)
list(APPEND LINKED_LIBS ${LINKED_LIBS} nlohmann_json::nlohmann_json)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Widgets ${LINKED_LIBS})
target_include_directories(${PROJECT_NAME} PRIVATE ${INCLUDES_DIR})

if(QT_VERSION_MAJOR EQUAL 6)
    qt_finalize_executable(${PROJECT_NAME})
endif()